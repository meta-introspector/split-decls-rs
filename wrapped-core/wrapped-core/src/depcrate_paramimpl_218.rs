// Generated macro for impl_218 (impl)
macro_rules! Depcrate_paramimpl_218 {
() => {
// Module: crate::param
// Provides: {"impl_218"}
// Dependencies: {}
impl < T > Param < T > for Option < & T > where T : Type < T > , { unsafe fn param (self) -> ParamValue < T > { unsafe { ParamValue :: Borrowed (match self { Some (item) => transmute_copy (item) , None => zeroed () , }) } } }
};
}
