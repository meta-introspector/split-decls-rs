// Generated macro for impl_35 (impl)
macro_rules! Depcrate_bundleimpl_35 {
() => {
// Module: crate::bundle
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a > ResourceBundle < 'a > { # [doc = " Makes a new resource bundle with the specified resource at its root."] pub fn new (name : Cow < 'a , str > , root : Resource < 'a > , is_locale_fallback_enabled : bool) -> Self { Self { name , root , is_locale_fallback_enabled , } } # [doc = " Gets the name of the resource bundle."] # [doc = ""] # [doc = " This name is used as the \"key\" of the root resource in a text format"] # [doc = " bundle, but is not used in building binary resource bundles."] pub fn name (& self) -> & str { & self . name } # [doc = " Gets the root resource in the resource tree."] pub fn root (& self) -> & Resource < '_ > { & self . root } }
};
}
