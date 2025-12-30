// Generated macro for extract_nparam (function)
macro_rules! Depcrate_deriveextract_nparam {
() => {
// Module: crate::derive
// Provides: {"extract_nparam"}
// Dependencies: {}
# [doc = " Wrap the given constructor with a let binding"] # [doc = " moving `param_<x>` into `params`."] fn extract_nparam < C > (acc : & mut PartsAcc < C > , params_ty : Type , (strat , ctor) : StratPair ,) -> StratPair { (strat , extract_api (ctor , FromReg :: Num (acc . add_param (params_ty))) ,) }
};
}
