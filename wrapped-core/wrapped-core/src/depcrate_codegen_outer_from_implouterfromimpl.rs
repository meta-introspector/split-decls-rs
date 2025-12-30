// Generated macro for OuterFromImpl (trait)
macro_rules! Depcrate_codegen_outer_from_implOuterFromImpl {
() => {
// Module: crate::codegen::outer_from_impl
// Provides: {"OuterFromImpl"}
// Dependencies: {}
# [doc = " Wrapper for \"outer From\" traits, such as `FromDeriveInput`, `FromVariant`, and `FromField`."] pub trait OuterFromImpl < 'a > { # [doc = " Gets the path of the trait being implemented."] fn trait_path (& self) -> Path ; fn base (& 'a self) -> & 'a TraitImpl < 'a > ; fn trait_bound (& self) -> Path { self . trait_path () } fn wrap < T : ToTokens > (& 'a self , body : T , tokens : & mut TokenStream) { let base = self . base () ; let trayt = self . trait_path () ; let ty_ident = base . ident ; let used = base . used_type_params () ; let generics = compute_impl_bounds (self . trait_bound () , base . generics . clone () , & used) ; let (impl_generics , ty_generics , where_clause) = generics . split_for_impl () ; tokens . append_all (quote ! (# [automatically_derived] # [allow (clippy :: manual_unwrap_or_default)] impl # impl_generics # trayt for # ty_ident # ty_generics # where_clause { # body })) ; } }
};
}
