// Generated macro for impl_147 (impl)
macro_rules! Depcrate_contextimpl_147 {
() => {
// Module: crate::context
// Provides: {"impl_147"}
// Dependencies: {}
impl < A : Actor < Context = Context < A > > > fmt :: Debug for Context < A > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Context") . field ("parts" , & self . parts) . field ("mb" , & self . mb) . finish () } }
};
}
