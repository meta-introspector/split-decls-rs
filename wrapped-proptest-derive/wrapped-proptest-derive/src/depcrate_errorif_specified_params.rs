// Generated macro for if_specified_params (function)
macro_rules! Depcrate_errorif_specified_params {
() => {
// Module: crate::error
// Provides: {"if_specified_params"}
// Dependencies: {}
# [doc = " Ensures that parameters is not present on `item`."] pub fn if_specified_params (ctx : Ctx , attrs : & ParsedAttributes , item : & str) { if attrs . params . is_set () { parent_has_param (ctx , item) ; } }
};
}
