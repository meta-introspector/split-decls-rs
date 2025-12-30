// Generated macro for if_specified_filter (function)
macro_rules! Depcrate_errorif_specified_filter {
() => {
// Module: crate::error
// Provides: {"if_specified_filter"}
// Dependencies: {}
# [doc = " Ensures that parameters is not present on `item`."] pub fn if_specified_filter (ctx : Ctx , attrs : & ParsedAttributes , item : & str) { if ! attrs . filter . is_empty () { meaningless_filter (ctx , item) ; } }
};
}
