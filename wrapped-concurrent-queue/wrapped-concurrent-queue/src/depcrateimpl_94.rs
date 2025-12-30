// Generated macro for impl_94 (impl)
macro_rules! Depcrateimpl_94 {
() => {
// Module: crate
// Provides: {"impl_94"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for ForcePushError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("ForcePushError") . field (& self . 0) . finish () } }
};
}
