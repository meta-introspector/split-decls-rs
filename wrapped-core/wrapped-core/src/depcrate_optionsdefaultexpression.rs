// Generated macro for DefaultExpression (enum)
macro_rules! Depcrate_optionsDefaultExpression {
() => {
// Module: crate::options
// Provides: {"DefaultExpression"}
// Dependencies: {}
# [doc = " A default/fallback expression encountered in attributes during parsing."] # [derive (Debug , Clone)] pub enum DefaultExpression { # [doc = " The value should be taken from the `default` instance of the containing struct."] # [doc = " This is not valid in container options."] Inherit , # [doc = " `default = path::to::function` or `default = || default_val()`."] Explicit (Callable) , Trait { # [doc = " The input span that is responsible for the use of `Default::default`."] span : Span , } , }
};
}
