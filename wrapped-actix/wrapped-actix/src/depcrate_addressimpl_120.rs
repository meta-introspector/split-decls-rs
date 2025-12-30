// Generated macro for impl_120 (impl)
macro_rules! Depcrate_addressimpl_120 {
() => {
// Module: crate::address
// Provides: {"impl_120"}
// Dependencies: {}
impl < A : Actor > fmt :: Debug for Addr < A > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Addr") . field ("tx" , & self . tx) . finish () } }
};
}
