// Generated macro for impl_221 (impl)
macro_rules! Depcrate_paramimpl_221 {
() => {
// Module: crate::param
// Provides: {"impl_221"}
// Dependencies: {}
impl < T > Param < T , CloneType > for & T where T : TypeKind < TypeKind = CloneType > + Clone , { unsafe fn param (self) -> ParamValue < T > { unsafe { ParamValue :: Borrowed (transmute_copy (self)) } } }
};
}
