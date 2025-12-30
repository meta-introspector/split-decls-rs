// Generated macro for parse_params_mode (function)
macro_rules! Depcrate_attrparse_params_mode {
() => {
// Module: crate::attr
// Provides: {"parse_params_mode"}
// Dependencies: {}
# [doc = " Combines a potentially set `params` and `no_params` into a single value"] # [doc = " and fails if both have been set. Only one of them can be set, or none."] fn parse_params_mode (ctx : Ctx , no_params : Option < () > , ty_params : Option < Type > ,) -> DeriveResult < ParamsMode > { Ok (match (no_params , ty_params) { (None , None) => ParamsMode :: Passthrough , (None , Some (ty)) => ParamsMode :: Specified (ty) , (Some (_) , None) => ParamsMode :: Default , (Some (_) , Some (_)) => error :: overspecified_param (ctx) ? , }) }
};
}
