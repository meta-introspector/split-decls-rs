// Generated macro for pack_progress_cb (function)
macro_rules! Depcrate_remote_callbackspack_progress_cb {
() => {
// Module: crate::remote_callbacks
// Provides: {"pack_progress_cb"}
// Dependencies: {}
extern "C" fn pack_progress_cb (stage : raw :: git_packbuilder_stage_t , current : c_uint , total : c_uint , data : * mut c_void ,) -> c_int { panic :: wrap (| | unsafe { let payload = & mut * (data as * mut RemoteCallbacks < '_ >) ; let callback = match payload . pack_progress { Some (ref mut c) => c , None => return 0 , } ; let stage = Binding :: from_raw (stage) ; callback (stage , current as usize , total as usize) ; 0 }) . unwrap_or (- 1) }
};
}
