// Generated macro for impl_222 (impl)
macro_rules! Depcrate_paramimpl_222 {
() => {
// Module: crate::param
// Provides: {"impl_222"}
// Dependencies: {}
impl < T , U > Param < T , CopyType > for U where T : TypeKind < TypeKind = CopyType > + Clone , U : TypeKind < TypeKind = CopyType > + Clone , U : imp :: CanInto < T > , { unsafe fn param (self) -> ParamValue < T > { unsafe { ParamValue :: Owned (transmute_copy (& self)) } } }
};
}
