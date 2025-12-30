// Generated macro for pkg_id (function)
macro_rules! Depcrate_helperspkg_id {
() => {
// Module: crate::helpers
// Provides: {"pkg_id"}
// Dependencies: {}
pub fn pkg_id (name : & str) -> PackageId { PackageId :: try_new (name , "1.0.0" , registry_loc ()) . unwrap () }
};
}
