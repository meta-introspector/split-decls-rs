// Generated macro for Span (enum)
macro_rules! Depcrate_errSpan {
() => {
// Module: crate::err
// Provides: {"Span"}
// Dependencies: {}
# [doc = " Unfortunately, we have to deal with both cases."] # [derive (Debug , Clone , Copy)] pub (crate) enum Span { One (proc_macro :: Span) , # [cfg (feature = "proc-macro2")] Two (proc_macro2 :: Span) , }
};
}
