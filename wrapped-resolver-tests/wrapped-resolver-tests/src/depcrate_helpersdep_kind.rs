// Generated macro for dep_kind (function)
macro_rules! Depcrate_helpersdep_kind {
() => {
// Module: crate::helpers
// Provides: {"dep_kind"}
// Dependencies: {}
pub fn dep_kind (name : & str , kind : DepKind) -> Dependency { let mut dep = dep (name) ; dep . set_kind (kind) ; dep }
};
}
