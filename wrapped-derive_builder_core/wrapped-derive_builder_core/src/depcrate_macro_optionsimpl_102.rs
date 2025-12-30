// Generated macro for impl_102 (impl)
macro_rules! Depcrate_macro_optionsimpl_102 {
() => {
// Module: crate::macro_options
// Provides: {"impl_102"}
// Dependencies: {}
impl FromMeta for BuildFnError { fn from_meta (item : & Meta) -> darling :: Result < Self > { match item { Meta :: Path (_) => Err (Error :: unsupported_format ("word") . with_span (item)) , Meta :: List (_) => BuildFnErrorGenerated :: from_meta (item) . map (Self :: Generated) , Meta :: NameValue (i) => Path :: from_expr (& i . value) . map (Self :: Existing) , } } }
};
}
