// Generated macro for impl_206 (impl)
macro_rules! Depcrate_out_paramimpl_206 {
() => {
// Module: crate::out_param
// Provides: {"impl_206"}
// Dependencies: {}
impl < T > OutParam < T > for Option < & mut T > where T : Type < T > , { unsafe fn borrow_mut (& self) -> OutRef < '_ , T > { unsafe { match self { Some (this) => transmute_copy (this) , None => zeroed () , } } } }
};
}
