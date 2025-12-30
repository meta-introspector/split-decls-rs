// Generated macro for default_setter (macro)
macro_rules! Depcrate_setterdefault_setter {
() => {
// Module: crate::setter
// Provides: {"default_setter"}
// Dependencies: {}
# [doc = " Helper macro for unit tests. This is _only_ public in order to be accessible"] # [doc = " from doc-tests too."] # [doc (hidden)] # [macro_export] macro_rules ! default_setter { () => { Setter { crate_root : & parse_quote ! (:: db) , setter_enabled : true , try_setter : false , visibility : :: std :: borrow :: Cow :: Owned (parse_quote ! (pub)) , pattern : BuilderPattern :: Mutable , attrs : & [] , ident : syn :: Ident :: new ("foo" , :: proc_macro2 :: Span :: call_site ()) , field_ident : & syn :: Ident :: new ("foo" , :: proc_macro2 :: Span :: call_site ()) , field_type : BuilderFieldType :: Optional (Box :: leak (Box :: new (parse_quote ! (Foo)))) , generic_into : false , strip_option : false , each : None , } } ; }
};
}
