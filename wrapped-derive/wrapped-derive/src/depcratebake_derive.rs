// Generated macro for bake_derive (function)
macro_rules! Depcratebake_derive {
() => {
// Module: crate
// Provides: {"bake_derive"}
// Dependencies: {}
# [doc = " This custom derive auto-implements the `Bake` trait on any given type that has public"] # [doc = " fields that also implement `Bake`."] # [doc = ""] # [doc = " For a type `Person` defined in the module `module` of crate `bar`, this derive"] # [doc = " can be used as follows:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use databake::Bake;"] # [doc = ""] # [doc = " #[derive(Bake)]"] # [doc = " #[databake(path = bar::module)]"] # [doc = " pub struct Person<'a> {"] # [doc = "     pub name: &'a str,"] # [doc = "     pub age: u32,"] # [doc = " }"] # [doc = " ```"] # [proc_macro_derive (Bake , attributes (databake))] pub fn bake_derive (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; TokenStream :: from (bake_derive_impl (& input)) }
};
}
