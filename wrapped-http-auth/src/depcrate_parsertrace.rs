// Generated macro for trace (macro)
macro_rules! Depcrate_parsertrace {
() => {
// Module: crate::parser
// Provides: {"trace"}
// Dependencies: {}
# [doc = " Calls `log::trace!` only if the `trace` cargo feature is enabled."] macro_rules ! trace { ($ ($ arg : tt) +) => (# [cfg (feature = "trace")] log :: trace ! ($ ($ arg) +)) }
};
}
