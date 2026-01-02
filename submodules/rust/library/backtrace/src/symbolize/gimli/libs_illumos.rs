mkuse!{use super :: mystd :: ffi :: OsStr ;}
mkuse!{use super :: mystd :: os :: unix :: prelude :: * ;}
mkuse!{use super :: { Library , LibrarySegment } ;}
mkuse!{use alloc :: borrow :: ToOwned ;}
mkuse!{use alloc :: vec :: Vec ;}
mkuse!{use core :: ffi :: CStr ;}
mkuse!{use core :: mem ;}
mkuse!{use object :: NativeEndian ;}
mkuse!{# [cfg (target_pointer_width = "64")] use object :: elf :: { FileHeader64 as FileHeader , ProgramHeader64 as ProgramHeader } ;}
mkitem!{type EHdr = FileHeader < NativeEndian > ;}
mkitem!{type PHdr = ProgramHeader < NativeEndian > ;}
mkitem!{mkstruct!{# [repr (C)] struct LinkMap { l_addr : libc :: c_ulong , l_name : * const libc :: c_char , l_ld : * const libc :: c_void , l_next : * const LinkMap , l_prev : * const LinkMap , l_refname : * const libc :: c_char , }}}
mkitem!{const RTLD_SELF : * const libc :: c_void = - 3isize as * const libc :: c_void ;}
mkitem!{const RTLD_DI_LINKMAP : libc :: c_int = 2 ;}
mkitem!{unsafe extern "C" { fn dlinfo (handle : * const libc :: c_void , request : libc :: c_int , p : * mut libc :: c_void ,) -> libc :: c_int ; }}

macro_rules! native_libraries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function native_libraries in module {}", module_path!());
    };
}

mkfn!{
    native_libraries_introspect!();
    pub (super) fn native_libraries () -> Vec < Library > { let mut libs = Vec :: new () ; let map = unsafe { let mut map : * const LinkMap = mem :: zeroed () ; if dlinfo (RTLD_SELF , RTLD_DI_LINKMAP , core :: ptr :: addr_of_mut ! (map) . cast :: < libc :: c_void > () ,) != 0 { return libs ; } map } ; let mut l = map ; while ! l . is_null () { let bytes = unsafe { CStr :: from_ptr ((* l) . l_name) } . to_bytes () ; let name = OsStr :: from_bytes (bytes) . to_owned () ; let addr = unsafe { (* l) . l_addr } ; let e : * const EHdr = unsafe { (* l) . l_addr as * const EHdr } ; let phoff = unsafe { (* e) . e_phoff } . get (NativeEndian) ; let phnum = unsafe { (* e) . e_phnum } . get (NativeEndian) ; let etype = unsafe { (* e) . e_type } . get (NativeEndian) ; let phdr : * const PHdr = (addr + phoff) as * const PHdr ; let phdr = unsafe { core :: slice :: from_raw_parts (phdr , phnum as usize) } ; libs . push (Library { name , segments : phdr . iter () . map (| p | { let memsz = p . p_memsz . get (NativeEndian) ; let vaddr = p . p_vaddr . get (NativeEndian) ; LibrarySegment { len : memsz as usize , stated_virtual_memory_address : vaddr as usize , } }) . collect () , bias : if etype == object :: elf :: ET_EXEC { 0 } else { addr as usize } , }) ; l = unsafe { (* l) . l_next } ; } libs }
}