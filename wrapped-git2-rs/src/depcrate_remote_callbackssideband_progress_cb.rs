// Generated macro for sideband_progress_cb (function)
macro_rules! Depcrate_remote_callbackssideband_progress_cb {
() => {
// Module: crate::remote_callbacks
// Provides: {"sideband_progress_cb"}
// Dependencies: {}
extern "C" fn sideband_progress_cb (str : * const c_char , len : c_int , payload : * mut c_void) -> c_int { let ok = panic :: wrap (| | unsafe { let payload = & mut * (payload as * mut RemoteCallbacks < '_ >) ; let callback = match payload . sideband_progress { Some (ref mut c) => c , None => return true , } ; let buf = slice :: from_raw_parts (str as * const u8 , len as usize) ; callback (buf) }) ; if ok == Some (true) { 0 } else { - 1 } }
};
}
