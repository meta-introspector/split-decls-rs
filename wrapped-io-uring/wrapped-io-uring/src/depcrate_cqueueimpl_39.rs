// Generated macro for impl_39 (impl)
macro_rules! Depcrate_cqueueimpl_39 {
() => {
// Module: crate::cqueue
// Provides: {"impl_39"}
// Dependencies: {}
impl Debug for Entry { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Entry") . field ("result" , & self . result ()) . field ("user_data" , & self . user_data ()) . field ("flags" , & self . flags ()) . finish () } }
};
}
