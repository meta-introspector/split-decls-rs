mkuse!{use super :: super :: super :: windows_sys :: * ;}
mkuse!{use super :: mystd :: ffi :: OsString ;}
mkuse!{use super :: { Library , LibrarySegment , coff , mmap } ;}
mkuse!{use alloc :: vec ;}
mkuse!{use alloc :: vec :: Vec ;}
mkuse!{use core :: mem ;}
mkuse!{use core :: mem :: MaybeUninit ;}

macro_rules! native_libraries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function native_libraries in module {}", module_path!());
    };
}

mkfn!{
    native_libraries_introspect!();
    pub (super) fn native_libraries () -> Vec < Library > { let mut ret = Vec :: new () ; unsafe { add_loaded_images (& mut ret) ; } return ret ; }
}

macro_rules! add_loaded_images_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_loaded_images in module {}", module_path!());
    };
}

mkfn!{
    add_loaded_images_introspect!();
    unsafe fn add_loaded_images (ret : & mut Vec < Library >) { unsafe { let snap = CreateToolhelp32Snapshot (TH32CS_SNAPMODULE , 0) ; if snap == INVALID_HANDLE_VALUE { return ; } let mut me = MaybeUninit :: < MODULEENTRY32W > :: zeroed () . assume_init () ; me . dwSize = mem :: size_of_val (& me) as u32 ; if Module32FirstW (snap , & mut me) == TRUE { loop { if let Some (lib) = load_library (& me) { ret . push (lib) ; } if Module32NextW (snap , & mut me) != TRUE { break ; } } } CloseHandle (snap) ; } }
}

macro_rules! get_posix_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_posix_path in module {}", module_path!());
    };
}

mkfn!{
    get_posix_path_introspect!();
    # [cfg (target_os = "cygwin")] unsafe fn get_posix_path (long_path : & [u16]) -> Option < OsString > { use super :: mystd :: os :: unix :: ffi :: OsStringExt ; unsafe extern "C" { fn cygwin_conv_path (what : libc :: c_uint , from : * const u16 , to : * mut u8 , size : libc :: size_t ,) -> libc :: ssize_t ; } const CCP_WIN_W_TO_POSIX : libc :: c_uint = 3 ; let name_len = unsafe { cygwin_conv_path (CCP_WIN_W_TO_POSIX , long_path . as_ptr () , core :: ptr :: null_mut () , 0 ,) } ; if name_len < 1 { return None ; } let name_len = name_len as usize ; let mut name_buffer = Vec :: with_capacity (name_len) ; let res = unsafe { cygwin_conv_path (CCP_WIN_W_TO_POSIX , long_path . as_ptr () , name_buffer . as_mut_ptr () , name_len ,) } ; if res != 0 { return None ; } unsafe { name_buffer . set_len (name_len - 1) } ; let name = OsString :: from_vec (name_buffer) ; Some (name) }
}

macro_rules! load_library_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function load_library in module {}", module_path!());
    };
}

mkfn!{
    load_library_introspect!();
    unsafe fn load_library (me : & MODULEENTRY32W) -> Option < Library > { # [cfg (windows)] let name = { use super :: mystd :: os :: windows :: prelude :: * ; let pos = me . szExePath . iter () . position (| i | * i == 0) . unwrap_or (me . szExePath . len ()) ; OsString :: from_wide (& me . szExePath [.. pos]) } ; # [cfg (target_os = "cygwin")] let name = unsafe { get_posix_path (& me . szExePath [..]) ? } ; let mmap = mmap (name . as_ref ()) ? ; let image_base = coff :: get_image_base (& mmap) ? ; let base_addr = me . modBaseAddr as usize ; Some (Library { name , bias : base_addr . wrapping_sub (image_base) , segments : vec ! [LibrarySegment { stated_virtual_memory_address : image_base , len : me . modBaseSize as usize , }] , }) }
}