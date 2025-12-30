// Generated macro for impl_554 (impl)
macro_rules! Depcrate_concurrency_threadimpl_554 {
() => {
// Module: crate::concurrency::thread
// Provides: {"impl_554"}
// Dependencies: {}
impl < 'tcx > std :: fmt :: Debug for ThreadState < 'tcx > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Enabled => write ! (f , "Enabled") , Self :: Blocked { reason , timeout , .. } => f . debug_struct ("Blocked") . field ("reason" , reason) . field ("timeout" , timeout) . finish () , Self :: Terminated => write ! (f , "Terminated") , } } }
};
}
