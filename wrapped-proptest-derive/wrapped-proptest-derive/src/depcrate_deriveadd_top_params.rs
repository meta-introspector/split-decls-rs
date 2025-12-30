// Generated macro for add_top_params (function)
macro_rules! Depcrate_deriveadd_top_params {
() => {
// Module: crate::derive
// Provides: {"add_top_params"}
// Dependencies: {}
# [doc = " Determine the `Parameters` part. We've already handled everything else."] # [doc = " After this, we have all parts needed for an impl. If `None` is given,"] # [doc = " then the unit type `()` will be used for `Parameters`."] fn add_top_params (param_ty : Option < Type > , (strat , ctor) : StratPair ,) -> ImplParts { let params = Params :: empty () ; if let Some (params_ty) = param_ty { (params + params_ty , strat , extract_api (ctor , FromReg :: Top)) } else { (params , strat , ctor) } }
};
}
