// Generated macro for derive_enum (function)
macro_rules! Depcrate_derivederive_enum {
() => {
// Module: crate::derive
// Provides: {"derive_enum"}
// Dependencies: {}
# [doc = " Entry point for deriving `Arbitrary` for `enum`s."] fn derive_enum (ctx : Ctx , mut ast : DeriveData < Vec < Variant > > ,) -> DeriveResult < Impl > { error :: if_skip_present (ctx , & ast . attrs , error :: ENUM) ; error :: if_strategy_present (ctx , & ast . attrs , error :: ENUM) ; error :: if_weight_present (ctx , & ast . attrs , error :: ENUM) ; if ast . body . is_empty () { error :: uninhabited_enum_with_no_variants (ctx) ? ; } if (& * ast . body) . is_uninhabited () { error :: uninhabited_enum_variants_uninhabited (ctx) ? ; } let parts = if let Some (sty) = ast . attrs . params . into_option () { derive_enum_has_params (ctx , & mut ast . tracker , & ast . ident , ast . body , sty) } else { derive_enum_no_params (ctx , & mut ast . tracker , & ast . ident , ast . body) } ? ; let parts = add_top_filter (ast . attrs . filter , parts) ; Ok (Impl :: new (ast . ident , ast . tracker , parts)) }
};
}
