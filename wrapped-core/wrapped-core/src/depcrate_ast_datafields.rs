// Generated macro for Fields (struct)
macro_rules! Depcrate_ast_dataFields {
() => {
// Module: crate::ast::data
// Provides: {"Fields"}
// Dependencies: {}
# [doc = " Equivalent to `syn::Fields`, but replaces the AST element with a generic."] # [derive (Debug , Clone)] pub struct Fields < T > { pub style : Style , pub fields : Vec < T > , span : Option < Span > , __nonexhaustive : () , }
};
}
