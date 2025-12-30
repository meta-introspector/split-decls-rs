// Generated macro for impl_497 (impl)
macro_rules! Depcrate_repository_implsimpl_497 {
() => {
// Module: crate::repository::impls
// Provides: {"impl_497"}
// Dependencies: {}
impl PartialEq < crate :: Repository > for crate :: Repository { fn eq (& self , other : & crate :: Repository) -> bool { self . git_dir () . canonicalize () . ok () == other . git_dir () . canonicalize () . ok () && self . work_tree . as_deref () . and_then (| wt | wt . canonicalize () . ok ()) == other . work_tree . as_deref () . and_then (| wt | wt . canonicalize () . ok ()) } }
};
}
