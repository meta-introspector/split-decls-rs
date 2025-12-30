// Generated macro for impl_219 (impl)
macro_rules! Depcrate_paramimpl_219 {
() => {
// Module: crate::param
// Provides: {"impl_219"}
// Dependencies: {}
impl < T > Param < T > for InterfaceRef < '_ , T > where T : Type < T > , { unsafe fn param (self) -> ParamValue < T > { unsafe { ParamValue :: Borrowed (transmute_copy (& self)) } } }
};
}
