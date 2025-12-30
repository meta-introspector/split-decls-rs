// Generated macro for impl_220 (impl)
macro_rules! Depcrate_paramimpl_220 {
() => {
// Module: crate::param
// Provides: {"impl_220"}
// Dependencies: {}
impl < T , U > Param < T , InterfaceType > for & U where T : TypeKind < TypeKind = InterfaceType > + Clone , T : Interface , U : Interface , U : imp :: CanInto < T > , { unsafe fn param (self) -> ParamValue < T > { unsafe { if U :: QUERY { self . cast () . map_or (ParamValue :: Borrowed (zeroed ()) , | ok | ParamValue :: Owned (ok)) } else { ParamValue :: Borrowed (transmute_copy (self)) } } } }
};
}
