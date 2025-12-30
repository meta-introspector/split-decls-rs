// Generated macro for pkg_id_loc (function)
macro_rules! Depcrate_helperspkg_id_loc {
() => {
// Module: crate::helpers
// Provides: {"pkg_id_loc"}
// Dependencies: {}
fn pkg_id_loc (name : & str , loc : & str) -> PackageId { let remote = loc . into_url () ; let master = GitReference :: Branch ("master" . to_string ()) ; let source_id = SourceId :: for_git (& remote . unwrap () , master) . unwrap () ; PackageId :: try_new (name , "1.0.0" , source_id) . unwrap () }
};
}
