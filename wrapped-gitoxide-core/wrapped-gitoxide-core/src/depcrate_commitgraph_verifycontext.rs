// Generated macro for Context (struct)
macro_rules! Depcrate_commitgraph_verifyContext {
() => {
// Module: crate::commitgraph::verify
// Provides: {"Context"}
// Dependencies: {}
# [doc = " A general purpose context for many operations provided here"] pub struct Context < W1 : std :: io :: Write , W2 : std :: io :: Write > { # [doc = " A stream to which to output errors"] pub err : W2 , # [doc = " A stream to which to output operation results"] pub out : W1 , pub output_statistics : Option < OutputFormat > , }
};
}
