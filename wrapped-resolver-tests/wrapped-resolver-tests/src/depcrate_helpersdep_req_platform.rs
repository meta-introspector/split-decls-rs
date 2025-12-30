// Generated macro for dep_req_platform (function)
macro_rules! Depcrate_helpersdep_req_platform {
() => {
// Module: crate::helpers
// Provides: {"dep_req_platform"}
// Dependencies: {}
pub fn dep_req_platform (name : & str , req : & str , platform : & str) -> Dependency { let mut dep = dep_req (name , req) ; dep . set_platform (Some (platform . parse () . unwrap ())) ; dep }
};
}
