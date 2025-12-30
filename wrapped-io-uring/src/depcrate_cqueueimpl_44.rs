// Generated macro for impl_44 (impl)
macro_rules! Depcrate_cqueueimpl_44 {
() => {
// Module: crate::cqueue
// Provides: {"impl_44"}
// Dependencies: {}
impl Debug for Entry32 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Entry32") . field ("result" , & self . result ()) . field ("user_data" , & self . user_data ()) . field ("flags" , & self . flags ()) . field ("big_cqe" , & self . big_cqe ()) . finish () } }
};
}
