// Generated macro for error_if_set (function)
macro_rules! Depcrate_attrerror_if_set {
() => {
// Module: crate::attr
// Provides: {"error_if_set"}
// Dependencies: {}
# [doc = " Emits a \"set again\" error iff the given option `.is_some()`."] fn error_if_set < T > (ctx : Ctx , loc : & Option < T > , meta : & Meta) { if loc . is_some () { error :: set_again (ctx , meta) } }
};
}
