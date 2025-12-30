// Generated macro for default_initializer (macro)
macro_rules! Depcrate_initializerdefault_initializer {
() => {
// Module: crate::initializer
// Provides: {"default_initializer"}
// Dependencies: {}
# [doc = " Helper macro for unit tests. This is _only_ public in order to be accessible"] # [doc = " from doc-tests too."] # [doc (hidden)] # [macro_export] macro_rules ! default_initializer { () => { Initializer { crate_root : & parse_quote ! (:: db) , field_ident : & syn :: Ident :: new ("foo" , :: proc_macro2 :: Span :: call_site ()) , field_enabled : true , builder_pattern : BuilderPattern :: Mutable , default_value : None , use_default_struct : false , conversion : FieldConversion :: OptionOrDefault , custom_error_type_span : None , } } ; }
};
}
