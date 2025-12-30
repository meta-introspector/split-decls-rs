// Generated macro for pkg_type (macro)
macro_rules! Depcrate_versionspkg_type {
() => {
// Module: crate::versions
// Provides: {"pkg_type"}
// Dependencies: {}
macro_rules ! pkg_type { ($ ($ variant : ident = $ component : literal $ (; preview = true $ (@$ is_preview : tt) ?) ?) ,+ $ (,) ?) => { # [derive (Debug , Hash , Eq , PartialEq , Clone)] pub (crate) enum PkgType { $ ($ variant ,) + } impl PkgType { pub (crate) fn is_preview (& self) -> bool { match self { $ ($ ($ ($ is_preview) ? PkgType ::$ variant => true ,) ?) + _ => false , } } # [doc = " First part of the tarball name."] pub (crate) fn tarball_component_name (& self) -> & str { match self { $ (PkgType ::$ variant => $ component ,) + } } pub (crate) fn all () -> &'static [PkgType] { & [$ (PkgType ::$ variant) ,+] } } } }
};
}
