// Generated macro for format_compact (macro)
macro_rules! Depcrate_macrosformat_compact {
() => {
// Module: crate::macros
// Provides: {"format_compact"}
// Dependencies: {}
# [doc = " Creates a `CompactString` using interpolation of runtime expressions."] # [doc = ""] # [doc = " The first argument `format_compact!` receives is a format string."] # [doc = " This must be a string literal."] # [doc = " The power of the formatting string is in the `{}`s contained."] # [doc = ""] # [doc = " Additional parameters passed to `format_compact!` replace the `{}`s within"] # [doc = " the formatting string in the order given unless named or"] # [doc = " positional parameters are used; see [`std::fmt`] for more information."] # [doc = ""] # [doc = " A common use for `format_compact!` is concatenation and interpolation"] # [doc = " of strings."] # [doc = " The same convention is used with [`print!`] and [`write!`] macros,"] # [doc = " depending on the intended destination of the string."] # [doc = ""] # [doc = " To convert a single value to a string, use the"] # [doc = " `ToCompactString::to_compact_string` method, which uses"] # [doc = " the [`std::fmt::Display`] formatting trait."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " `format_compact!` panics if a formatting trait implementation returns"] # [doc = " an error."] # [doc = ""] # [doc = " This indicates an incorrect implementation since"] # [doc = " `ToCompactString::to_compact_string` never returns an error itself."] # [macro_export] macro_rules ! format_compact { ($ ($ arg : tt) *) => { $ crate :: ToCompactString :: to_compact_string (&$ crate :: core :: format_args ! ($ ($ arg) *)) } }
};
}
