// Generated macro for names (function)
macro_rules! Depcrate_helpersnames {
() => {
// Module: crate::helpers
// Provides: {"names"}
// Dependencies: {}
pub fn names < P : ToPkgId > (names : & [P]) -> Vec < PackageId > { names . iter () . map (| name | name . to_pkgid ()) . collect () }
};
}
