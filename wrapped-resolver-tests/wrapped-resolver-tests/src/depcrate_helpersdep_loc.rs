// Generated macro for dep_loc (function)
macro_rules! Depcrate_helpersdep_loc {
() => {
// Module: crate::helpers
// Provides: {"dep_loc"}
// Dependencies: {}
pub fn dep_loc (name : & str , location : & str) -> Dependency { let url = location . into_url () . unwrap () ; let master = GitReference :: Branch ("master" . to_string ()) ; let source_id = SourceId :: for_git (& url , master) . unwrap () ; Dependency :: parse (name , Some ("1.0.0") , source_id) . unwrap () }
};
}
