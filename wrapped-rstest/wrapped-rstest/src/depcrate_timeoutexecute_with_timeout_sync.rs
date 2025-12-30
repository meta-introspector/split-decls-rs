// Generated macro for execute_with_timeout_sync (function)
macro_rules! Depcrate_timeoutexecute_with_timeout_sync {
() => {
// Module: crate::timeout
// Provides: {"execute_with_timeout_sync"}
// Dependencies: {}
pub fn execute_with_timeout_sync < T : 'static + Send , F : FnOnce () -> T + Send + 'static > (code : F , timeout : Duration ,) -> T { let (sender , receiver) = mpsc :: channel () ; let thread = if let Some (name) = thread :: current () . name () { thread :: Builder :: new () . name (name . to_string ()) } else { thread :: Builder :: new () } ; let handle = thread . spawn (move | | sender . send (code ())) . unwrap () ; match receiver . recv_timeout (timeout) { Ok (result) => { handle . join () . unwrap () . unwrap () ; result } Err (mpsc :: RecvTimeoutError :: Timeout) => panic ! ("Timeout {timeout:?} expired") , Err (mpsc :: RecvTimeoutError :: Disconnected) => match handle . join () { Err (any) => std :: panic :: resume_unwind (any) , Ok (_) => unreachable ! () , } , } }
};
}
