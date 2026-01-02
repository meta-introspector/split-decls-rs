mkuse!{use super :: mystd :: env ;}
mkuse!{use super :: mystd :: ffi :: { OsStr , OsString } ;}
mkuse!{use super :: mystd :: os :: unix :: prelude :: * ;}
mkuse!{use super :: { Library , LibrarySegment , parse_running_mmaps } ;}
mkuse!{use alloc :: borrow :: ToOwned ;}
mkuse!{use alloc :: vec :: Vec ;}
mkuse!{use core :: ffi :: CStr ;}
mkuse!{use core :: slice ;}
mkitem!{mkstruct!{struct CallbackData { libs : Vec < Library > , maps : Option < Vec < parse_running_mmaps :: MapsEntry > > , }}}

macro_rules! native_libraries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function native_libraries in module {}", module_path!());
    };
}

mkfn!{
    native_libraries_introspect!();
    pub (super) fn native_libraries () -> Vec < Library > { let mut cb_data = CallbackData { libs : Vec :: new () , # [cfg (not (target_os = "hurd"))] maps : parse_running_mmaps :: parse_maps () . ok () , # [cfg (target_os = "hurd")] maps : None , } ; unsafe { libc :: dl_iterate_phdr (Some (callback) , core :: ptr :: addr_of_mut ! (cb_data) . cast ()) ; } cb_data . libs }
}

macro_rules! infer_current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function infer_current_exe in module {}", module_path!());
    };
}

mkfn!{
    infer_current_exe_introspect!();
    fn infer_current_exe (maps : & Option < Vec < parse_running_mmaps :: MapsEntry > > , base_addr : usize ,) -> OsString { # [cfg (not (target_os = "hurd"))] if let Some (entries) = maps { let opt_path = entries . iter () . find (| e | e . ip_matches (base_addr) && e . pathname () . len () > 0) . map (| e | e . pathname ()) . cloned () ; if let Some (path) = opt_path { return path ; } } env :: current_exe () . map (| e | e . into ()) . unwrap_or_default () }
}

macro_rules! callback_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function callback in module {}", module_path!());
    };
}

mkfn!{
    callback_introspect!();
    # [doc = " # Safety"] # [doc = " `info` must be a valid pointer."] # [doc = " `data` must be a valid pointer to `CallbackData`."] # [forbid (unsafe_op_in_unsafe_fn)] unsafe extern "C" fn callback (info : * mut libc :: dl_phdr_info , _size : libc :: size_t , data : * mut libc :: c_void ,) -> libc :: c_int { let dlpi_addr = unsafe { (* info) . dlpi_addr } ; let dlpi_name = unsafe { (* info) . dlpi_name } ; let dlpi_phdr = unsafe { (* info) . dlpi_phdr } ; let dlpi_phnum = unsafe { (* info) . dlpi_phnum } ; let CallbackData { libs , maps } = unsafe { & mut * data . cast :: < CallbackData > () } ; let is_main = libs . is_empty () ; let is_static = dlpi_addr == 0 ; let no_given_name = dlpi_name . is_null () || unsafe { * dlpi_name == 0 } ; let name = if is_static { env :: current_exe () . unwrap_or_default () . into_os_string () } else if is_main && no_given_name { infer_current_exe (& maps , dlpi_addr as usize) } else { if dlpi_name . is_null () { OsString :: new () } else { OsStr :: from_bytes (unsafe { CStr :: from_ptr (dlpi_name) } . to_bytes ()) . to_owned () } } ; # [cfg (target_os = "android")] let zip_offset : Option < u64 > = { maps . as_ref () . and_then (| maps | { super :: extract_zip_path_android (& name) . and_then (| _ | { maps . iter () . find (| m | m . ip_matches (dlpi_addr as usize)) . map (| m | m . offset ()) }) }) } ; let headers = if dlpi_phdr . is_null () || dlpi_phnum == 0 { & [] } else { unsafe { slice :: from_raw_parts (dlpi_phdr , dlpi_phnum as usize) } } ; libs . push (Library { name , # [cfg (target_os = "android")] zip_offset , segments : headers . iter () . map (| header | LibrarySegment { len : header . p_memsz as usize , stated_virtual_memory_address : header . p_vaddr as usize , }) . collect () , bias : dlpi_addr as usize , }) ; 0 }
}