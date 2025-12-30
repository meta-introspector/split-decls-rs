// Generated macro for impl_19 (impl)
macro_rules! Depcrate_astimpl_19 {
() => {
// Module: crate::ast
// Provides: {"impl_19"}
// Dependencies: {}
impl Impl { # [doc = " Constructs a new `Impl` from the parts as described on the type."] pub fn new (typ : syn :: Ident , tracker : UseTracker , parts : ImplParts) -> Self { Self { typ , tracker , parts , } } # [doc = " Linearises the impl into a sequence of tokens."] # [doc = " This produces the actual Rust code for the impl."] pub fn into_tokens (self , ctx : Ctx) -> DeriveResult < TokenStream > { let Impl { typ , mut tracker , parts : (params , strategy , ctor) , } = self ; # [doc = " A `Debug` bound on a type variable."] fn debug_bound () -> syn :: TypeParamBound { parse_quote ! (:: std :: fmt :: Debug) } # [doc = " An `Arbitrary` bound on a type variable."] fn arbitrary_bound () -> syn :: TypeParamBound { parse_quote ! (_proptest :: arbitrary :: Arbitrary) } tracker . add_bounds (ctx , & arbitrary_bound () , Some (debug_bound ())) ? ; let generics = tracker . consume () ; let (impl_generics , ty_generics , where_clause) = generics . split_for_impl () ; let _top = call_site_ident (TOP_PARAM_NAME) ; let q = quote ! { # [allow (non_local_definitions)] # [allow (non_upper_case_globals)] # [allow (clippy :: arc_with_non_send_sync)] const _ : () = { use proptest as _proptest ; impl # impl_generics _proptest :: arbitrary :: Arbitrary for # typ # ty_generics # where_clause { type Parameters = # params ; type Strategy = # strategy ; fn arbitrary_with (# _top : Self :: Parameters) -> Self :: Strategy { # ctor } } } ; } ; Ok (q) } }
};
}
