// Generated macro for dispatch_inner (macro)
macro_rules! Depcrate_parser_choicedispatch_inner {
() => {
// Module: crate::parser::choice
// Provides: {"dispatch_inner"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! dispatch_inner { ($ expr_ident : ident [$ first_ident : ident $ ($ id : ident) *] [$ ($ collected : tt) *] $ ($ pat : pat) |+ $ (if $ pred : expr) ? => $ expr : expr , $ ($ rest_alt : tt) *) => { $ crate :: dispatch_inner ! { $ expr_ident [$ ($ id) *] [$ ($ collected) * $ first_ident $ ($ pat) |+ $ (if $ pred) ? => $ expr ,] $ ($ rest_alt) * } } ; ($ expr_ident : ident [$ ($ id : ident) *] [$ ($ collected : tt) *]) => { $ crate :: dispatch_inner ! { $ expr_ident $ ($ collected) * } } ; ($ expr_ident : ident [$ ($ ident_tt : tt) *]) => { unreachable ! () } ; ($ expr_ident : ident $ ($ ident : ident $ ($ pat : pat) |+ $ (if $ pred : expr) ? => $ expr : expr ,) +) => { match $ expr_ident { $ ($ ($ pat) |+ $ (if $ pred) ? => Dispatch ::$ ident (check_parser ($ expr)) ,) + } } }
};
}
