// Generated macro for if_skip_present (function)
macro_rules! Depcrate_errorif_skip_present {
() => {
// Module: crate::error
// Provides: {"if_skip_present"}
// Dependencies: {}
# [doc = " Ensures that skip is not present on `item`."] pub fn if_skip_present (ctx : Ctx , attrs : & ParsedAttributes , item : & str) { if attrs . skip { illegal_skip (ctx , item) } }
};
}
