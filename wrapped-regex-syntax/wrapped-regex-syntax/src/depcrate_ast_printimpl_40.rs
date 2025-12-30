// Generated macro for impl_40 (impl)
macro_rules! Depcrate_ast_printimpl_40 {
() => {
// Module: crate::ast::print
// Provides: {"impl_40"}
// Dependencies: {}
impl Printer { # [doc = " Create a new printer."] pub fn new () -> Printer { PrinterBuilder :: new () . build () } # [doc = " Print the given `Ast` to the given writer. The writer must implement"] # [doc = " `fmt::Write`. Typical implementations of `fmt::Write` that can be used"] # [doc = " here are a `fmt::Formatter` (which is available in `fmt::Display`"] # [doc = " implementations) or a `&mut String`."] pub fn print < W : fmt :: Write > (& mut self , ast : & Ast , wtr : W) -> fmt :: Result { visitor :: visit (ast , Writer { wtr }) } }
};
}
