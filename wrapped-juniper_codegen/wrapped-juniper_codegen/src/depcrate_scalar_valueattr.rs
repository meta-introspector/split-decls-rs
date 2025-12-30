// Generated macro for Attr (struct)
macro_rules! Depcrate_scalar_valueAttr {
() => {
// Module: crate::scalar_value
// Provides: {"Attr"}
// Dependencies: {}
# [doc = " Available arguments behind `#[value]` attribute when generating code for"] # [doc = " an enum definition."] # [derive (Default)] struct Attr { # [doc = " Explicitly specified function to be used as `ScalarValue::from_displayable()`"] # [doc = " implementation."] from_displayable : Option < SpanContainer < syn :: ExprPath > > , # [doc = " Explicitly specified function to be used as `ScalarValue::from_displayable_non_static()`"] # [doc = " implementation."] from_displayable_non_static : Option < SpanContainer < syn :: ExprPath > > , }
};
}
