// Generated macro for FormatArgsStorage (struct)
macro_rules! Depcrate_macrosFormatArgsStorage {
() => {
// Module: crate::macros
// Provides: {"FormatArgsStorage"}
// Dependencies: {}
# [doc = " Stores AST [`FormatArgs`] nodes for use in late lint passes, as they are in a desugared form in"] # [doc = " the HIR"] # [derive (Default , Clone)] pub struct FormatArgsStorage (Arc < OnceLock < FxHashMap < Span , FormatArgs > > >) ;
};
}
