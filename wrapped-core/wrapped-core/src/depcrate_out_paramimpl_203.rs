// Generated macro for impl_203 (impl)
macro_rules! Depcrate_out_paramimpl_203 {
() => {
// Module: crate::out_param
// Provides: {"impl_203"}
// Dependencies: {}
impl < T > OutParam < T , CloneType > for & mut T where T : TypeKind < TypeKind = CloneType > + Clone + Default , { unsafe fn borrow_mut (& self) -> OutRef < '_ , T > { unsafe { let this : & mut T = transmute_copy (self) ; take (this) ; transmute_copy (self) } } }
};
}
