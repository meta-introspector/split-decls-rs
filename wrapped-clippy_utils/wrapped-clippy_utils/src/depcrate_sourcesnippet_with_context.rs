// Generated macro for snippet_with_context (function)
macro_rules! Depcrate_sourcesnippet_with_context {
() => {
// Module: crate::source
// Provides: {"snippet_with_context"}
// Dependencies: {}
# [doc = " Same as `snippet_with_applicability`, but first walks the span up to the given context."] # [doc = ""] # [doc = " This will result in the macro call, rather than the expansion, if the span is from a child"] # [doc = " context. If the span is not from a child context, it will be used directly instead."] # [doc = ""] # [doc = " e.g. Given the expression `&vec![]`, getting a snippet from the span for `vec![]` as a HIR node"] # [doc = " would result in `box []`. If given the context of the address of expression, this function will"] # [doc = " correctly get a snippet of `vec![]`."] # [doc = ""] # [doc = " This will also return whether or not the snippet is a macro call."] pub fn snippet_with_context < 'a > (sess : & impl HasSession , span : Span , outer : SyntaxContext , default : & 'a str , applicability : & mut Applicability ,) -> (Cow < 'a , str > , bool) { snippet_with_context_sess (sess . sess () , span , outer , default , applicability) }
};
}
