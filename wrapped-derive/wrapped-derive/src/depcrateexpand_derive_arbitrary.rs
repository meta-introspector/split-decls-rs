// Generated macro for expand_derive_arbitrary (function)
macro_rules! Depcrateexpand_derive_arbitrary {
() => {
// Module: crate
// Provides: {"expand_derive_arbitrary"}
// Dependencies: {}
fn expand_derive_arbitrary (input : syn :: DeriveInput) -> Result < TokenStream > { let container_attrs = ContainerAttributes :: from_derive_input (& input) ? ; let (lifetime_without_bounds , lifetime_with_bounds) = build_arbitrary_lifetime (input . generics . clone ()) ; let recursive_count = syn :: Ident :: new (& format ! ("RECURSIVE_COUNT_{}" , input . ident . unraw ()) , Span :: call_site () ,) ; let (arbitrary_method , needs_recursive_count) = gen_arbitrary_method (& input , lifetime_without_bounds . clone () , & recursive_count) ? ; let size_hint_method = gen_size_hint_method (& input , needs_recursive_count) ? ; let name = input . ident ; let generics = apply_trait_bounds (input . generics , lifetime_without_bounds . clone () , & container_attrs ,) ? ; let mut generics_with_lifetime = generics . clone () ; generics_with_lifetime . params . push (GenericParam :: Lifetime (lifetime_with_bounds)) ; let (impl_generics , _ , _) = generics_with_lifetime . split_for_impl () ; let (_ , ty_generics , where_clause) = generics . split_for_impl () ; let recursive_count = needs_recursive_count . then (| | { Some (quote ! { :: std :: thread_local ! { # [allow (non_upper_case_globals)] static # recursive_count : :: core :: cell :: Cell < u32 > = const { :: core :: cell :: Cell :: new (0) } ; } }) }) ; Ok (quote ! { const _ : () = { # recursive_count # [automatically_derived] impl # impl_generics arbitrary :: Arbitrary <# lifetime_without_bounds > for # name # ty_generics # where_clause { # arbitrary_method # size_hint_method } } ; }) }
};
}
