// Generated macro for pkg_id_source (function)
macro_rules! Depcrate_helperspkg_id_source {
() => {
// Module: crate::helpers
// Provides: {"pkg_id_source"}
// Dependencies: {}
pub fn pkg_id_source (name : & str , source : & str) -> PackageId { PackageId :: try_new (name , "1.0.0" , SourceId :: for_registry (& source . into_url () . unwrap ()) . unwrap () ,) . unwrap () }
};
}
