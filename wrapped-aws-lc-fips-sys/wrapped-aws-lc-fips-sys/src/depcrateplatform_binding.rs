// Generated macro for platform_binding (macro)
macro_rules! Depcrateplatform_binding {
() => {
// Module: crate
// Provides: {"platform_binding"}
// Dependencies: {}
macro_rules ! platform_binding { ($ platform : ident , $ platform_crypto : ident , $ platform_ssl : ident) => { # [cfg (all ($ platform , not (feature = "ssl") , not (use_bindgen_generated)))] use_bindings ! ($ platform_crypto) ; # [cfg (all ($ platform , feature = "ssl" , not (use_bindgen_generated)))] use_bindings ! ($ platform_ssl) ; } ; }
};
}
