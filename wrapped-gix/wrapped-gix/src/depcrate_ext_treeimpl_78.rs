// Generated macro for impl_78 (impl)
macro_rules! Depcrate_ext_treeimpl_78 {
() => {
// Module: crate::ext::tree
// Provides: {"impl_78"}
// Dependencies: {}
impl TreeIterExt for TreeRefIter < '_ > { fn traverse < StateMut , Find , V > (& self , state : StateMut , objects : Find , delegate : & mut V ,) -> Result < () , breadthfirst :: Error > where Find : gix_object :: Find , StateMut : BorrowMut < breadthfirst :: State > , V : gix_traverse :: tree :: Visit , { breadthfirst (* self , state , objects , delegate) } }
};
}
