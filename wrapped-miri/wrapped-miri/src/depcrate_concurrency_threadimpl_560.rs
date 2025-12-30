// Generated macro for impl_560 (impl)
macro_rules! Depcrate_concurrency_threadimpl_560 {
() => {
// Module: crate::concurrency::thread
// Provides: {"impl_560"}
// Dependencies: {}
impl < 'tcx > std :: fmt :: Debug for Thread < 'tcx > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}({:?}, {:?})" , String :: from_utf8_lossy (self . thread_name () . unwrap_or (b"<unnamed>")) , self . state , self . join_status) } }
};
}
