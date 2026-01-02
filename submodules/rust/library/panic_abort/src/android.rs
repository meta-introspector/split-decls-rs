mkuse!{use alloc :: string :: String ;}
mkuse!{use core :: mem :: transmute ;}
mkuse!{use core :: panic :: PanicPayload ;}
mkuse!{use core :: ptr :: copy_nonoverlapping ;}
mkitem!{const ANDROID_SET_ABORT_MESSAGE : & [u8] = b"android_set_abort_message\0" ;}
mkitem!{type SetAbortMessageType = unsafe extern "C" fn (* const libc :: c_char) -> () ;}

macro_rules! android_set_abort_message_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function android_set_abort_message in module {}", module_path!());
    };
}

mkfn!{
    android_set_abort_message_introspect!();
    pub (crate) unsafe fn android_set_abort_message (payload : & mut dyn PanicPayload) { let func_addr = unsafe { libc :: dlsym (libc :: RTLD_DEFAULT , ANDROID_SET_ABORT_MESSAGE . as_ptr () as * const libc :: c_char) as usize } ; if func_addr == 0 { return ; } let payload = payload . get () ; let msg = match payload . downcast_ref :: < & 'static str > () { Some (msg) => msg . as_bytes () , None => match payload . downcast_ref :: < String > () { Some (msg) => msg . as_bytes () , None => & [] , } , } ; if msg . is_empty () { return ; } let size = msg . len () + 1usize ; let buf = unsafe { libc :: malloc (size) as * mut libc :: c_char } ; if buf . is_null () { return ; } unsafe { copy_nonoverlapping (msg . as_ptr () , buf as * mut u8 , msg . len ()) ; buf . add (msg . len ()) . write (0) ; let func = transmute :: < usize , SetAbortMessageType > (func_addr) ; func (buf) ; } }
}