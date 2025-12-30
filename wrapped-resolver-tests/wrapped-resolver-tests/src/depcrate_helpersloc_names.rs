// Generated macro for loc_names (function)
macro_rules! Depcrate_helpersloc_names {
() => {
// Module: crate::helpers
// Provides: {"loc_names"}
// Dependencies: {}
pub fn loc_names (names : & [(& 'static str , & 'static str)]) -> Vec < PackageId > { names . iter () . map (| & (name , loc) | pkg_id_loc (name , loc)) . collect () }
};
}
