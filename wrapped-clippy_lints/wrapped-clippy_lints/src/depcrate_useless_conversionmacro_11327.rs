// Generated macro for macro_11327 (macro)
macro_rules! Depcrate_useless_conversionmacro_11327 {
() => {
// Module: crate::useless_conversion
// Provides: {"macro_11327"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `Into`, `TryInto`, `From`, `TryFrom`, or `IntoIter` calls"] # [doc = " which uselessly convert to the same type."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Redundant code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " // format!() returns a `String`"] # [doc = " let s: String = format!(\"hello\").into();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let s: String = format!(\"hello\");"] # [doc = " ```"] # [clippy :: version = "1.45.0"] pub USELESS_CONVERSION , complexity , "calls to `Into`, `TryInto`, `From`, `TryFrom`, or `IntoIter` which perform useless conversions to the same type" }
};
}
