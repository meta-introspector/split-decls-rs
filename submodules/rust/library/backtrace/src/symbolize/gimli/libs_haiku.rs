mkuse!{use super :: mystd :: ffi :: OsStr ;}
mkuse!{use super :: mystd :: os :: unix :: prelude :: * ;}
mkuse!{use super :: { Library , LibrarySegment } ;}
mkuse!{use alloc :: borrow :: ToOwned ;}
mkuse!{use alloc :: vec :: Vec ;}
mkuse!{use core :: ffi :: CStr ;}
mkuse!{use core :: mem :: MaybeUninit ;}

macro_rules! native_libraries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function native_libraries in module {}", module_path!());
    };
}

mkfn!{
    native_libraries_introspect!();
    pub (super) fn native_libraries () -> Vec < Library > { let mut libraries : Vec < Library > = Vec :: new () ; unsafe { let mut info = MaybeUninit :: < libc :: image_info > :: zeroed () ; let mut cookie : i32 = 0 ; let mut status = libc :: get_next_image_info (libc :: B_CURRENT_TEAM , & mut cookie , info . as_mut_ptr ()) ; if status != libc :: B_OK { return libraries ; } let mut info = info . assume_init () ; while status == libc :: B_OK { let mut segments = Vec :: new () ; segments . push (LibrarySegment { stated_virtual_memory_address : 0 , len : info . text_size as usize , }) ; let bytes = CStr :: from_ptr (info . name . as_ptr ()) . to_bytes () ; let name = OsStr :: from_bytes (bytes) . to_owned () ; libraries . push (Library { name : name , segments : segments , bias : info . text as usize , }) ; status = libc :: get_next_image_info (libc :: B_CURRENT_TEAM , & mut cookie , & mut info) ; } } libraries }
}