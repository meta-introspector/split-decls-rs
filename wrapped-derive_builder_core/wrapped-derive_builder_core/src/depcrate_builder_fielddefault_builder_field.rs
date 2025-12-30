// Generated macro for default_builder_field (macro)
macro_rules! Depcrate_builder_fielddefault_builder_field {
() => {
// Module: crate::builder_field
// Provides: {"default_builder_field"}
// Dependencies: {}
# [doc = " Helper macro for unit tests. This is _only_ public in order to be accessible"] # [doc = " from doc-tests too."] # [cfg (test)] # [doc (hidden)] # [macro_export] macro_rules ! default_builder_field { () => { { BuilderField { crate_root : & parse_quote ! (:: db) , field_ident : & syn :: Ident :: new ("foo" , :: proc_macro2 :: Span :: call_site ()) , field_type : BuilderFieldType :: Optional (Box :: leak (Box :: new (parse_quote ! (String)))) , field_visibility : :: std :: borrow :: Cow :: Owned (parse_quote ! (pub)) , attrs : & [parse_quote ! (# [some_attr])] , } } } ; }
};
}
