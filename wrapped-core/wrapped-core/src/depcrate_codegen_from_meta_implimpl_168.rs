// Generated macro for impl_168 (impl)
macro_rules! Depcrate_codegen_from_meta_implimpl_168 {
() => {
// Module: crate::codegen::from_meta_impl
// Provides: {"impl_168"}
// Dependencies: {}
impl ToTokens for ParseImpl < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let from_meta = self . 0 . trait_path () ; let impl_block = quote ! { fn parse (input : :: darling :: export :: syn :: parse :: ParseStream <'_ >) -> :: darling :: export :: syn :: Result < Self > { use :: darling :: export :: IntoIterator ; let items = :: darling :: export :: syn :: punctuated :: Punctuated ::<:: darling :: export :: NestedMeta , :: darling :: export :: syn :: Token ! [,] >:: parse_terminated (input) ? . into_iter () . collect ::<:: darling :: export :: Vec < _ >> () ; < Self as # from_meta >:: from_list (& items) . map_err (:: darling :: export :: Into :: into) } } ; self . wrap (impl_block , tokens) ; } }
};
}
