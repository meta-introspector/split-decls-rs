// Generated macro for derive_enum_no_params (function)
macro_rules! Depcrate_derivederive_enum_no_params {
() => {
// Module: crate::derive
// Provides: {"derive_enum_no_params"}
// Dependencies: {}
# [doc = " Deriving for a enum on which `params` or `no_params` was NOT set directly."] fn derive_enum_no_params (ctx : Ctx , ut : & mut UseTracker , _self : & Ident , variants : Vec < Variant > ,) -> DeriveResult < ImplParts > { let mut acc = PartsAcc :: new (variants . len ()) ; for variant in variants { if let Some ((weight , ident , fields , attrs)) = keep_inhabited_variant (ctx , _self , variant) ? { let path = parse_quote ! (# _self ::# ident) ; let (strat , ctor) = if fields . is_empty () { pair_unit_variant (ctx , & attrs , path) } else { derive_variant_with_fields (ctx , ut , path , attrs , fields , & mut acc ,) ? } ; acc = acc . add_strat ((strat , (weight , ctor))) ; } } ensure_union_has_strategies (ctx , & acc . strats) ; Ok (acc . finish (ctx)) }
};
}
