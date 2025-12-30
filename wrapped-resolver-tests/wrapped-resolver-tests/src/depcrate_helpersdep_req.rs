// Generated macro for dep_req (function)
macro_rules! Depcrate_helpersdep_req {
() => {
// Module: crate::helpers
// Provides: {"dep_req"}
// Dependencies: {}
pub fn dep_req (name : & str , req : & str) -> Dependency { Dependency :: parse (name , Some (req) , registry_loc ()) . unwrap () }
};
}
