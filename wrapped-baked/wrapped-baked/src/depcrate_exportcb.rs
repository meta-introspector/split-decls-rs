// Generated macro for cb (macro)
macro_rules! Depcrate_exportcb {
() => {
// Module: crate::export
// Provides: {"cb"}
// Dependencies: {}
macro_rules ! cb { ($ ($ marker_ty : ty :$ marker : ident ,) + # [experimental] $ ($ emarker_ty : ty :$ emarker : ident ,) +) => { fn bake_marker (marker : DataMarkerInfo) -> databake :: TokenStream { if marker . id == icu_provider :: hello_world :: HelloWorldV1 :: INFO . id { return databake :: quote ! (icu_provider :: hello_world :: HelloWorldV1) ; } $ (if marker . id . name () == stringify ! ($ marker) { return stringify ! ($ marker_ty) . replace ("icu :: " , "icu_") . parse () . unwrap () ; }) + $ (if marker . id . name () == stringify ! ($ emarker) { return stringify ! ($ emarker_ty) . replace ("icu :: " , "icu_") . parse () . unwrap () ; }) + unreachable ! ("unregistered marker {marker:?}") } } }
};
}
