// Generated macro for build (function)
macro_rules! Depcratebuild {
() => {
// Module: crate
// Provides: {"build"}
// Dependencies: {}
fn build (attr : TokenStream , item : TokenStream) -> Result < TokenStream > { let mut item : Item = parse2 (item) ? ; let ts = match & mut item { Item :: Struct (item_struct) => item_type :: build_by_item_struct (attr , item_struct) , Item :: Enum (item_enum) => item_type :: build_by_item_enum (attr , item_enum) , Item :: Impl (item_impl) => item_impl :: build_by_item_impl (attr , item_impl) , _ => bail ! (_ , "`#[derive_ex]` can be specified only for `struct`, `enum`, or `impl`." ,) , } . unwrap_or_else (| e | e . to_compile_error ()) ; Ok (quote ! (# item # ts)) }
};
}
