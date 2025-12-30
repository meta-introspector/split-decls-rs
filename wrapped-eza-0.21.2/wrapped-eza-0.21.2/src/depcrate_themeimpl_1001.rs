// Generated macro for impl_1001 (impl)
macro_rules! Depcrate_themeimpl_1001 {
() => {
// Module: crate::theme
// Provides: {"impl_1001"}
// Dependencies: {}
impl FileStyle for ExtensionMappings { fn get_style (& self , file : & File < '_ > , _theme : & Theme) -> Option < Style > { let maybe_ext = file . name . rsplit_once ('.') . map (| x | x . 1) ; for mapping in self . mappings . iter () . rev () { match mapping { GlobPattern :: Complex (pat , style) => { if pat . matches (& file . name) { return Some (* style) ; } } GlobPattern :: Simple (map) => { if let Some (ext) = maybe_ext { if let Some (style) = map . get (ext) { return Some (* style) ; } } } } } None } }
};
}
