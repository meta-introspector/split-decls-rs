// Generated macro for NormMeta (enum)
macro_rules! Depcrate_attrNormMeta {
() => {
// Module: crate::attr
// Provides: {"NormMeta"}
// Dependencies: {}
# [doc = " Normalized `Meta` into all the forms we will possibly accept."] # [derive (Debug)] enum NormMeta { # [doc = " Accepts: `#[proptest(<word>)]`"] Plain , # [doc = " Accepts: `#[proptest(<word> = <lit>)]` and `#[proptest(<word>(<lit>))]`"] Lit (Lit) , # [doc = " Accepts: `#[proptest(<word>(<word>))`."] Word (Ident) , }
};
}
