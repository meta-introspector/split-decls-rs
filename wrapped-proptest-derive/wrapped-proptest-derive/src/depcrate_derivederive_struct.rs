// Generated macro for derive_struct (function)
macro_rules! Depcrate_derivederive_struct {
() => {
// Module: crate::derive
// Provides: {"derive_struct"}
// Dependencies: {}
# [doc = " Entry point for deriving `Arbitrary` for `struct`s."] fn derive_struct (ctx : Ctx , mut ast : DeriveData < Vec < Field > > ,) -> DeriveResult < Impl > { error :: if_enum_attrs_present (ctx , & ast . attrs , error :: STRUCT) ; error :: if_strategy_present (ctx , & ast . attrs , error :: STRUCT) ; let v_path = ast . ident . clone () . into () ; let parts = if ast . body . is_empty () { error :: if_present_on_unit_struct (ctx , & ast . attrs) ; let (strat , ctor) = pair_unit_self (& v_path) ; (Params :: empty () , strat , ctor) } else { if (& * ast . body) . is_uninhabited () { error :: uninhabited_struct (ctx) ; } let closure = map_closure (v_path , & ast . body) ; let parts = if let Some (param_ty) = ast . attrs . params . into_option () { add_top_params (param_ty , derive_product_has_params (ctx , & mut ast . tracker , error :: STRUCT_FIELD , closure , ast . body ,) ? ,) } else { derive_product_no_params (ctx , & mut ast . tracker , ast . body , error :: STRUCT_FIELD ,) ? . finish (closure) } ; add_top_filter (ast . attrs . filter , parts) } ; Ok (Impl :: new (ast . ident , ast . tracker , parts)) }
};
}
