// Generated macro for pkg_dep (function)
macro_rules! Depcrate_helperspkg_dep {
() => {
// Module: crate::helpers
// Provides: {"pkg_dep"}
// Dependencies: {}
pub fn pkg_dep < T : ToPkgId > (name : T , dep : Vec < Dependency >) -> Summary { let pkgid = name . to_pkgid () ; let link = if pkgid . name () . ends_with ("-sys") { Some (pkgid . name ()) } else { None } ; Summary :: new (name . to_pkgid () , dep , & BTreeMap :: new () , link , None) . unwrap () }
};
}
