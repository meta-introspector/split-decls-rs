// Generated macro for try_merge_opt (macro)
macro_rules! Depcratetry_merge_opt {
() => {
// Module: crate
// Provides: {"try_merge_opt"}
// Dependencies: {}
# [doc = " Attempts to merge an [`Option`]ed `$field` of a `$self` struct with the same `$field` of"] # [doc = " `$another` struct. If both are [`Some`], then throws a duplication error with a [`Span`] related"] # [doc = " to the `$another` struct (a later one)."] # [doc = ""] # [doc = " The type of [`Span`] may be explicitly specified as one of the [`SpanContainer`] methods."] # [doc = " By default, [`SpanContainer::span_ident`] is used."] # [doc = ""] # [doc = " [`Span`]: proc_macro2::Span"] # [doc = " [`SpanContainer`]: crate::common::SpanContainer"] # [doc = " [`SpanContainer::span_ident`]: crate::common::SpanContainer::span_ident"] macro_rules ! try_merge_opt { ($ field : ident : $ self : ident , $ another : ident => $ span : ident) => { { if let Some (v) = $ self .$ field { $ another .$ field . replace (v) . none_or_else (| dup | crate :: common :: parse :: attr :: err :: dup_arg (& dup .$ span ())) ?; } $ another .$ field } } ; ($ field : ident : $ self : ident , $ another : ident) => { try_merge_opt ! ($ field : $ self , $ another => span_ident) } ; }
};
}
