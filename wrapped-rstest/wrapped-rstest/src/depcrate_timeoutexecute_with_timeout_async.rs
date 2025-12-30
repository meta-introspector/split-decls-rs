// Generated macro for execute_with_timeout_async (function)
macro_rules! Depcrate_timeoutexecute_with_timeout_async {
() => {
// Module: crate::timeout
// Provides: {"execute_with_timeout_async"}
// Dependencies: {}
# [cfg (feature = "async-timeout")] pub async fn execute_with_timeout_async < T , Fut : Future < Output = T > , F : FnOnce () -> Fut > (code : F , timeout : Duration ,) -> T { let timeout_fut = pin ! (Delay :: new (timeout)) ; let code_fut = pin ! (code ()) ; match select (timeout_fut , code_fut) . await { Either :: Left ((() , _)) => { panic ! ("Timeout {:?} expired" , timeout) } Either :: Right ((out , _)) => out , } }
};
}
