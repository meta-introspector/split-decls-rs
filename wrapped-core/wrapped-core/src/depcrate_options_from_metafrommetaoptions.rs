// Generated macro for FromMetaOptions (struct)
macro_rules! Depcrate_options_from_metaFromMetaOptions {
() => {
// Module: crate::options::from_meta
// Provides: {"FromMetaOptions"}
// Dependencies: {}
pub struct FromMetaOptions { base : Core , # [doc = " Override for the default [`FromMeta::from_word`] method."] from_word : Option < Callable > , # [doc = " Override for the default [`FromMeta::from_none`] method."] from_none : Option < Callable > , # [doc = " Override for the default [`FromMeta::from_expr`] method."] from_expr : Option < Callable > , # [doc = " Whether or not to derive [`syn::parse::Parse`] in addition to deriving [`FromMeta`]."] derive_syn_parse : Option < bool > , }
};
}
