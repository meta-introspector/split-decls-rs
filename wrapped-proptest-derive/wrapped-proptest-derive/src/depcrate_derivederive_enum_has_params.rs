// Generated macro for derive_enum_has_params (function)
macro_rules! Depcrate_derivederive_enum_has_params {
() => {
// Module: crate::derive
// Provides: {"derive_enum_has_params"}
// Dependencies: {}
# [doc = " Derive for a variant which has fields and where the"] # [doc = " variant or its fields may NOT specify `params` or `no_params`."] fn derive_enum_has_params (ctx : Ctx , ut : & mut UseTracker , _self : & Ident , variants : Vec < Variant > , sty : Option < Type > ,) -> DeriveResult < ImplParts > { let mut acc = StratAcc :: new (variants . len ()) ; for variant in variants { let parts = keep_inhabited_variant (ctx , _self , variant) ? ; if let Some ((weight , ident , fields , attrs)) = parts { let path = parse_quote ! (# _self ::# ident) ; let (strat , ctor) = if fields . is_empty () { pair_unit_variant (ctx , & attrs , path) } else { let filter = attrs . filter . clone () ; add_filter_self (filter , variant_handle_default_params (ctx , ut , path , attrs , fields ,) ? ,) } ; acc = acc . add ((strat , (weight , ctor))) ; } } ensure_union_has_strategies (ctx , & acc) ; Ok (add_top_params (sty , acc . finish (ctx))) }
};
}
