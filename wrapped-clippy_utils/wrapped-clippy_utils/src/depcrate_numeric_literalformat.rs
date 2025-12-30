// Generated macro for format (function)
macro_rules! Depcrate_numeric_literalformat {
() => {
// Module: crate::numeric_literal
// Provides: {"format"}
// Dependencies: {}
# [doc = " A helper method to format numeric literals with digit grouping."] # [doc = " `lit` must be a valid numeric literal without suffix."] pub fn format (lit : & str , type_suffix : Option < & str > , float : bool) -> String { NumericLiteral :: new (lit , type_suffix , float) . format () }
};
}
