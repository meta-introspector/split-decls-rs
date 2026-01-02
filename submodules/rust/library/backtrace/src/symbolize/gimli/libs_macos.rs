mkuse!{use super :: mystd :: ffi :: OsStr ;}
mkuse!{use super :: mystd :: os :: unix :: prelude :: * ;}
mkuse!{use super :: mystd :: prelude :: v1 :: * ;}
mkuse!{use super :: { Library , LibrarySegment } ;}
mkuse!{use core :: convert :: TryInto ;}
mkuse!{use core :: ffi :: CStr ;}
mkuse!{use core :: mem ;}

macro_rules! ptr_from_ref_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ptr_from_ref in module {}", module_path!());
    };
}

mkfn!{
    ptr_from_ref_introspect!();
    # [inline (always)] # [must_use] const fn ptr_from_ref < T : ? Sized > (r : & T) -> * const T { r }
}

macro_rules! native_libraries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function native_libraries in module {}", module_path!());
    };
}

mkfn!{
    native_libraries_introspect!();
    pub (super) fn native_libraries () -> Vec < Library > { let mut ret = Vec :: new () ; let images = unsafe { libc :: _dyld_image_count () } ; for i in 0 .. images { ret . extend (native_library (i)) ; } return ret ; }
}

macro_rules! native_library_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function native_library in module {}", module_path!());
    };
}

mkfn!{
    native_library_introspect!();
    fn native_library (i : u32) -> Option < Library > { use object :: NativeEndian ; use object :: macho ; use object :: read :: macho :: { MachHeader , Segment } ; let name = unsafe { let name = libc :: _dyld_get_image_name (i) ; if name . is_null () { return None ; } CStr :: from_ptr (name) } ; let (mut load_commands , endian) = unsafe { let header = libc :: _dyld_get_image_header (i) ; if header . is_null () { return None ; } match (* header) . magic { macho :: MH_MAGIC => { let endian = NativeEndian ; let header = & * header . cast :: < macho :: MachHeader32 < NativeEndian > > () ; let data = core :: slice :: from_raw_parts (ptr_from_ref (header) . cast :: < u8 > () , mem :: size_of_val (header) + header . sizeofcmds . get (endian) as usize ,) ; (header . load_commands (endian , data , 0) . ok () ? , endian) } macho :: MH_MAGIC_64 => { let endian = NativeEndian ; let header = & * header . cast :: < macho :: MachHeader64 < NativeEndian > > () ; let data = core :: slice :: from_raw_parts (ptr_from_ref (header) . cast :: < u8 > () , mem :: size_of_val (header) + header . sizeofcmds . get (endian) as usize ,) ; (header . load_commands (endian , data , 0) . ok () ? , endian) } _ => return None , } } ; let mut segments = Vec :: new () ; let mut first_text = 0 ; let mut text_fileoff_zero = false ; while let Some (cmd) = load_commands . next () . ok () ? { if let Some ((seg , _)) = cmd . segment_32 () . ok () ? { if seg . name () == b"__TEXT" { first_text = segments . len () ; if seg . fileoff (endian) == 0 && seg . filesize (endian) > 0 { text_fileoff_zero = true ; } } segments . push (LibrarySegment { len : seg . vmsize (endian) . try_into () . ok () ? , stated_virtual_memory_address : seg . vmaddr (endian) . try_into () . ok () ? , }) ; } if let Some ((seg , _)) = cmd . segment_64 () . ok () ? { if seg . name () == b"__TEXT" { first_text = segments . len () ; if seg . fileoff (endian) == 0 && seg . filesize (endian) > 0 { text_fileoff_zero = true ; } } segments . push (LibrarySegment { len : seg . vmsize (endian) . try_into () . ok () ? , stated_virtual_memory_address : seg . vmaddr (endian) . try_into () . ok () ? , }) ; } } let mut slide = unsafe { libc :: _dyld_get_image_vmaddr_slide (i) as usize } ; if ! text_fileoff_zero { let adjust = segments [first_text] . stated_virtual_memory_address ; for segment in segments . iter_mut () { segment . stated_virtual_memory_address -= adjust ; } slide += adjust ; } Some (Library { name : OsStr :: from_bytes (name . to_bytes ()) . to_owned () , segments , bias : slide , }) }
}