// Generated macro for Printer (struct)
macro_rules! Depcrate_ast_printPrinter {
() => {
// Module: crate::ast::print
// Provides: {"Printer"}
// Dependencies: {}
# [doc = " A printer for a regular expression abstract syntax tree."] # [doc = ""] # [doc = " A printer converts an abstract syntax tree (AST) to a regular expression"] # [doc = " pattern string. This particular printer uses constant stack space and heap"] # [doc = " space proportional to the size of the AST."] # [doc = ""] # [doc = " This printer will not necessarily preserve the original formatting of the"] # [doc = " regular expression pattern string. For example, all whitespace and comments"] # [doc = " are ignored."] # [derive (Debug)] pub struct Printer { _priv : () , }
};
}
