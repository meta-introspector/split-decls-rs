// Generated macro for impl_204 (impl)
macro_rules! Depcrate_out_paramimpl_204 {
() => {
// Module: crate::out_param
// Provides: {"impl_204"}
// Dependencies: {}
impl < T > OutParam < T , CopyType > for & mut T where T : TypeKind < TypeKind = CopyType > + Clone + Default , { unsafe fn borrow_mut (& self) -> OutRef < '_ , T > { unsafe { transmute_copy (self) } } }
};
}
