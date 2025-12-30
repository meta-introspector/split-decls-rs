// Generated macro for do_derive_max_size (function)
macro_rules! Depcrate_max_sizedo_derive_max_size {
() => {
// Module: crate::max_size
// Provides: {"do_derive_max_size"}
// Dependencies: {}
pub fn do_derive_max_size (item : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let input = parse_macro_input ! (item as DeriveInput) ; let span = input . span () ; let name = input . ident ; let generics = add_trait_bounds (input . generics) ; let (impl_generics , ty_generics , where_clause) = generics . split_for_impl () ; let sum = max_size_sum (& input . data , span) . unwrap_or_else (syn :: Error :: into_compile_error) ; let expanded = quote ! { impl # impl_generics :: postcard :: experimental :: max_size :: MaxSize for # name # ty_generics # where_clause { const POSTCARD_MAX_SIZE : usize = # sum ; } } ; expanded . into () }
};
}
