// Generated macro for android_set_abort_message (function)
macro_rules! Depcrate_androidandroid_set_abort_message {
() => {
// Module: crate::android
// Provides: {"android_set_abort_message"}
// Dependencies: {}
pub (crate) unsafe fn android_set_abort_message (payload : & mut dyn PanicPayload) { let func_addr = unsafe { libc :: dlsym (libc :: RTLD_DEFAULT , ANDROID_SET_ABORT_MESSAGE . as_ptr () as * const libc :: c_char) as usize } ; if func_addr == 0 { return ; } let payload = payload . get () ; let msg = match payload . downcast_ref :: < & 'static str > () { Some (msg) => msg . as_bytes () , None => match payload . downcast_ref :: < String > () { Some (msg) => msg . as_bytes () , None => & [] , } , } ; if msg . is_empty () { return ; } let size = msg . len () + 1usize ; let buf = unsafe { libc :: malloc (size) as * mut libc :: c_char } ; if buf . is_null () { return ; } unsafe { copy_nonoverlapping (msg . as_ptr () , buf as * mut u8 , msg . len ()) ; buf . add (msg . len ()) . write (0) ; let func = transmute :: < usize , SetAbortMessageType > (func_addr) ; func (buf) ; } }
};
}
