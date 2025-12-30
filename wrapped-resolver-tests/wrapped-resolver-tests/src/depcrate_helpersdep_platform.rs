// Generated macro for dep_platform (function)
macro_rules! Depcrate_helpersdep_platform {
() => {
// Module: crate::helpers
// Provides: {"dep_platform"}
// Dependencies: {}
pub fn dep_platform (name : & str , platform : & str) -> Dependency { let mut dep = dep (name) ; dep . set_platform (Some (platform . parse () . unwrap ())) ; dep }
};
}
