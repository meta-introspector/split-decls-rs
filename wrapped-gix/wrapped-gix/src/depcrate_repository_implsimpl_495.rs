// Generated macro for impl_495 (impl)
macro_rules! Depcrate_repository_implsimpl_495 {
() => {
// Module: crate::repository::impls
// Provides: {"impl_495"}
// Dependencies: {}
impl Clone for crate :: Repository { fn clone (& self) -> Self { let mut new = crate :: Repository :: from_refs_and_objects (self . refs . clone () , self . objects . clone () , self . work_tree . clone () , self . common_dir . clone () , self . config . clone () , self . options . clone () , # [cfg (feature = "index")] self . index . clone () , self . shallow_commits . clone () , # [cfg (feature = "attributes")] self . modules . clone () ,) ; if self . bufs . is_none () { new . bufs . take () ; } new } }
};
}
