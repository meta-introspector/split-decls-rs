// Generated macro for default_build_method (macro)
macro_rules! Depcrate_build_methoddefault_build_method {
() => {
// Module: crate::build_method
// Provides: {"default_build_method"}
// Dependencies: {}
# [doc = " Helper macro for unit tests. This is _only_ public in order to be accessible"] # [doc = " from doc-tests too."] # [doc (hidden)] # [macro_export] macro_rules ! default_build_method { () => { BuildMethod { crate_root : & parse_quote ! (:: db) , enabled : true , ident : & syn :: Ident :: new ("build" , :: proc_macro2 :: Span :: call_site ()) , visibility : :: std :: borrow :: Cow :: Owned (syn :: parse_quote ! (pub)) , pattern : BuilderPattern :: Mutable , target_ty : & syn :: Ident :: new ("Foo" , :: proc_macro2 :: Span :: call_site ()) , target_ty_generics : None , error_ty : syn :: parse_quote ! (FooBuilderError) , initializers : vec ! [quote ! (foo : self . foo ,)] , doc_comment : None , default_struct : None , validate_fn : None , } } ; }
};
}
