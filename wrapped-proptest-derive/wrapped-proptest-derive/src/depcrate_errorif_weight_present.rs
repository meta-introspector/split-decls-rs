// Generated macro for if_weight_present (function)
macro_rules! Depcrate_errorif_weight_present {
() => {
// Module: crate::error
// Provides: {"if_weight_present"}
// Dependencies: {}
# [doc = " Ensures that a weight is not present on `item`."] pub fn if_weight_present (ctx : Ctx , attrs : & ParsedAttributes , item : & str) { if attrs . weight . is_some () { illegal_weight (ctx , item) } }
};
}
