// Generated macro for pkg_dep_with (function)
macro_rules! Depcrate_helperspkg_dep_with {
() => {
// Module: crate::helpers
// Provides: {"pkg_dep_with"}
// Dependencies: {}
pub fn pkg_dep_with < T : ToPkgId > (name : T , dep : Vec < Dependency > , features : & [(& 'static str , & [& 'static str])] ,) -> Summary { let pkgid = name . to_pkgid () ; let link = if pkgid . name () . ends_with ("-sys") { Some (pkgid . name ()) } else { None } ; let features = features . into_iter () . map (| & (name , values) | (name . into () , values . into_iter () . map (| & v | v . into ()) . collect ())) . collect () ; Summary :: new (name . to_pkgid () , dep , & features , link , None) . unwrap () }
};
}
