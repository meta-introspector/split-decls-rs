// Generated macro for private (module)
macro_rules! Depcrate_expression_methods_text_expression_methodsprivate {
() => {
// Module: crate::expression_methods::text_expression_methods
// Provides: {"private"}
// Dependencies: {}
mod private { use crate :: sql_types :: { Nullable , Text } ; # [doc = " Marker trait used to implement `TextExpressionMethods` on the appropriate"] # [doc = " types. Once coherence takes associated types into account, we can remove"] # [doc = " this trait."] pub trait TextOrNullableText { } impl TextOrNullableText for Text { } impl TextOrNullableText for Nullable < Text > { } }
};
}
