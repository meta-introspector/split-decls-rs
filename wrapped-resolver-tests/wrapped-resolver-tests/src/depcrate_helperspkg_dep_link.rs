// Generated macro for pkg_dep_link (function)
macro_rules! Depcrate_helperspkg_dep_link {
() => {
// Module: crate::helpers
// Provides: {"pkg_dep_link"}
// Dependencies: {}
pub fn pkg_dep_link < T : ToPkgId > (name : T , link : & str , dep : Vec < Dependency >) -> Summary { Summary :: new (name . to_pkgid () , dep , & BTreeMap :: new () , Some (link) , None) . unwrap () }
};
}
