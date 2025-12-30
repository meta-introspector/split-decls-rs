// Generated macro for dep_req_kind (function)
macro_rules! Depcrate_helpersdep_req_kind {
() => {
// Module: crate::helpers
// Provides: {"dep_req_kind"}
// Dependencies: {}
pub fn dep_req_kind (name : & str , req : & str , kind : DepKind) -> Dependency { let mut dep = dep_req (name , req) ; dep . set_kind (kind) ; dep }
};
}
