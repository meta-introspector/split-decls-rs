// Generated macro for LinkToModule (struct)
macro_rules! Depcrate_astLinkToModule {
() => {
// Module: crate::ast
// Provides: {"LinkToModule"}
// Dependencies: {}
# [doc = " An abstract syntax tree representing a link to a module in Rust."] # [doc = " In contrast to Program, LinkToModule must expand to an expression."] # [doc = " linked_modules of the inner Program must contain exactly one element"] # [doc = " whose link is produced by the expression."] # [cfg_attr (feature = "extra-traits" , derive (Debug))] # [derive (Clone)] pub struct LinkToModule (pub Program) ;
};
}
