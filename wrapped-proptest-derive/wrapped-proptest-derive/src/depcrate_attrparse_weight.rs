// Generated macro for parse_weight (function)
macro_rules! Depcrate_attrparse_weight {
() => {
// Module: crate::attr
// Provides: {"parse_weight"}
// Dependencies: {}
# [doc = " Parses a weight."] # [doc = " Valid forms are:"] # [doc = " + `#[proptest(weight = <integer>)]`"] # [doc = " + `#[proptest(weight = \"<expr>\")]`"] # [doc = " + `#[proptest(weight(<integer>))]`"] # [doc = " + `#[proptest(weight(\"<expr>\"\"))]`"] # [doc = ""] # [doc = " The `<integer>` must also fit within an `u32` and be unsigned."] fn parse_weight (ctx : Ctx , acc : & mut ParseAcc , meta : & Meta) { use std :: u32 ; error_if_set (ctx , & acc . weight , & meta) ; let value = normalize_meta (meta . clone ()) . and_then (extract_lit) . and_then (extract_expr) . as_ref () . and_then (interp :: eval_expr) . filter (| & value | value <= u128 :: from (u32 :: MAX)) . map (| value | value as u32) ; if let v @ Some (_) = value { acc . weight = v ; } else { error :: weight_malformed (ctx , meta) } }
};
}
