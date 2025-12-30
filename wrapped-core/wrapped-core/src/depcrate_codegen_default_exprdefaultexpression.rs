// Generated macro for DefaultExpression (enum)
macro_rules! Depcrate_codegen_default_exprDefaultExpression {
() => {
// Module: crate::codegen::default_expr
// Provides: {"DefaultExpression"}
// Dependencies: {}
# [doc = " The fallback value for a field or container."] # [derive (Debug , Clone)] pub enum DefaultExpression < 'a > { # [doc = " Only valid on fields, `Inherit` indicates that the value should be taken from a pre-constructed"] # [doc = " fallback object. The value in the variant is the ident of the field."] Inherit (& 'a Ident) , # [doc = " `default = path::to::function` or `default = || default_val()`."] Explicit (& 'a Callable) , Trait { span : Span , } , }
};
}
