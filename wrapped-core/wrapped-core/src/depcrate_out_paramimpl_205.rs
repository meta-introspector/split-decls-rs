// Generated macro for impl_205 (impl)
macro_rules! Depcrate_out_paramimpl_205 {
() => {
// Module: crate::out_param
// Provides: {"impl_205"}
// Dependencies: {}
impl < T > OutParam < T , InterfaceType > for & mut Option < T > where T : TypeKind < TypeKind = InterfaceType > + Clone , { unsafe fn borrow_mut (& self) -> OutRef < '_ , T > { unsafe { let this : & mut Option < T > = transmute_copy (self) ; take (this) ; transmute_copy (self) } } }
};
}
