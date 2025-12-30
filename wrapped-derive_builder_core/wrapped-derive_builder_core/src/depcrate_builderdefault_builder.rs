// Generated macro for default_builder (macro)
macro_rules! Depcrate_builderdefault_builder {
() => {
// Module: crate::builder
// Provides: {"default_builder"}
// Dependencies: {}
# [doc = " Helper macro for unit tests. This is _only_ public in order to be accessible"] # [doc = " from doc-tests too."] # [doc (hidden)] # [macro_export] macro_rules ! default_builder { () => { Builder { crate_root : & parse_quote ! (:: db) , enabled : true , ident : syn :: Ident :: new ("FooBuilder" , :: proc_macro2 :: Span :: call_site ()) , pattern : Default :: default () , derives : & [] , struct_attrs : & [] , impl_attrs : & [] , impl_default : true , create_empty : syn :: Ident :: new ("create_empty" , :: proc_macro2 :: Span :: call_site ()) , generics : None , visibility : :: std :: borrow :: Cow :: Owned (parse_quote ! (pub)) , fields : vec ! [quote ! (foo : u32 ,)] , field_initializers : vec ! [quote ! (foo : :: db :: export :: core :: default :: Default :: default () ,)] , functions : vec ! [quote ! (fn bar () -> { unimplemented ! () })] , generate_error : true , generate_validation_error : true , no_alloc : false , must_derive_clone : true , doc_comment : None , std : true , } } ; }
};
}
