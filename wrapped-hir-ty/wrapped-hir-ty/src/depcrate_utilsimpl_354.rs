// Generated macro for impl_354 (impl)
macro_rules! Depcrate_utilsimpl_354 {
() => {
// Module: crate::utils
// Provides: {"impl_354"}
// Dependencies: {}
impl < 'a > ClosureSubst < 'a > { pub (crate) fn parent_subst (& self) -> & 'a [GenericArg] { match self . 0 . as_slice (Interner) { [x @ .. , _] => x , _ => { never ! ("Closure missing parameter") ; & [] } } } pub (crate) fn sig_ty (& self) -> & 'a Ty { match self . 0 . as_slice (Interner) { [.. , x] => x . assert_ty_ref (Interner) , _ => { unreachable ! ("Closure missing sig_ty parameter") ; } } } }
};
}
