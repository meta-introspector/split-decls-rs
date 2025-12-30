// Generated macro for impl_462 (impl)
macro_rules! Depcrate_uri_portimpl_462 {
() => {
// Module: crate::uri::port
// Provides: {"impl_462"}
// Dependencies: {}
impl < T > fmt :: Debug for Port < T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Port") . field (& self . port) . finish () } }
};
}
