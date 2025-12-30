// Generated macro for variant_no_explicit_strategy (function)
macro_rules! Depcrate_derivevariant_no_explicit_strategy {
() => {
// Module: crate::derive
// Provides: {"variant_no_explicit_strategy"}
// Dependencies: {}
# [doc = " Derive for a variant on which params were not set and on which no explicit"] # [doc = " strategy was set (or where it doesn't make sense...) and which has fields."] fn variant_no_explicit_strategy < C > (ctx : Ctx , ut : & mut UseTracker , v_path : Path , fields : Vec < Field > , acc : & mut PartsAcc < C > ,) -> DeriveResult < StratPair > { let closure = map_closure (v_path , & fields) ; let fields_acc = derive_product_no_params (ctx , ut , fields , error :: ENUM_VARIANT_FIELD) ? ; let (params , count) = fields_acc . params . consume () ; let (strat , ctor) = fields_acc . strats . finish (closure) ; let params_ty = params . into () ; Ok ((strat , if is_unit_type (& params_ty) { ctor } else { let pref = acc . add_param (params_ty) ; extract_all (ctor , count , FromReg :: Num (pref)) } ,)) }
};
}
