// Generated macro for OutOfOrderParams (struct)
macro_rules! Depcrate_errorsOutOfOrderParams {
() => {
// Module: crate::errors
// Provides: {"OutOfOrderParams"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_out_of_order_params)] pub (crate) struct OutOfOrderParams < 'a > { # [primary_span] pub spans : Vec < Span > , # [suggestion (code = "{ordered_params}" , applicability = "machine-applicable")] pub sugg_span : Span , pub param_ord : & 'a ParamKindOrd , pub max_param : & 'a ParamKindOrd , pub ordered_params : & 'a str , }
};
}
