// Generated macro for wait_process (function)
macro_rules! Depcrate_processwait_process {
() => {
// Module: crate::process
// Provides: {"wait_process"}
// Dependencies: {}
fn wait_process (proc : HANDLE , timeout_millis : Option < u32 >) -> Result < u32 , Error > { match timeout_millis { Some (timeout) => { let result = unsafe { WaitForSingleObject (proc , timeout) } ; if result == WAIT_TIMEOUT { return Err (Error :: Timeout (Duration :: from_millis (timeout as u64))) ; } } None => match unsafe { WaitForSingleObject (proc , INFINITE) } { WAIT_OBJECT_0 => { } event_id => return Err (Error :: WaitFailed (event_id)) , } , } let mut code = 0 ; unsafe { GetExitCodeProcess (proc , & mut code) ? ; } Ok (code) }
};
}
