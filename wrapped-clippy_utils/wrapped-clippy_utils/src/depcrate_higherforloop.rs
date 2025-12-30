// Generated macro for ForLoop (struct)
macro_rules! Depcrate_higherForLoop {
() => {
// Module: crate::higher
// Provides: {"ForLoop"}
// Dependencies: {}
# [doc = " The essential nodes of a desugared for loop as well as the entire span:"] # [doc = " `for pat in arg { body }` becomes `(pat, arg, body)`. Returns `(pat, arg, body, span)`."] # [derive (Debug)] pub struct ForLoop < 'tcx > { # [doc = " `for` loop item"] pub pat : & 'tcx Pat < 'tcx > , # [doc = " `IntoIterator` argument"] pub arg : & 'tcx Expr < 'tcx > , # [doc = " `for` loop body"] pub body : & 'tcx Expr < 'tcx > , # [doc = " Compare this against `hir::Destination.target`"] pub loop_id : HirId , # [doc = " entire `for` loop span"] pub span : Span , # [doc = " label"] pub label : Option < ast :: Label > , }
};
}
