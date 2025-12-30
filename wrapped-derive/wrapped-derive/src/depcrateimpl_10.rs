// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl Prefix { # [doc = " Returns the path prefix to use for functions from the given trait."] fn for_trait (& self , trait_name : & str) -> TokenStream2 { let trait_name = Ident :: new (trait_name , Span :: call_site ()) ; match self { Prefix :: Type (ty) => quote ! { <# ty as tls_codec ::# trait_name > } , Prefix :: Custom (p) => quote ! { # p } , } } }
};
}
