// Generated macro for clone_stream (function)
macro_rules! Depcrateclone_stream {
() => {
// Module: crate
// Provides: {"clone_stream"}
// Dependencies: {}
fn clone_stream (ts : TokenStream) -> TokenStream { ts . into_iter () . map (clone_tree) . collect () }
};
}
