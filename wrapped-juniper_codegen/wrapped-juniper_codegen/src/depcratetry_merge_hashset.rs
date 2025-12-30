// Generated macro for try_merge_hashset (macro)
macro_rules! Depcratetry_merge_hashset {
() => {
// Module: crate
// Provides: {"try_merge_hashset"}
// Dependencies: {}
# [doc = " Attempts to merge a [`HashSet`] `$field` of a `$self` struct with the same `$field` of"] # [doc = " `$another` struct. If some [`HashSet`] entries are duplicated, then throws a duplication error"] # [doc = " with a [`Span`] related to the `$another` struct (a later one)."] # [doc = ""] # [doc = " The type of [`Span`] may be explicitly specified as one of the [`SpanContainer`] methods."] # [doc = " By default, [`SpanContainer::span_ident`] is used."] # [doc = ""] # [doc = " [`HashSet`]: std::collections::HashSet"] # [doc = " [`Span`]: proc_macro2::Span"] # [doc = " [`SpanContainer`]: crate::common::SpanContainer"] # [doc = " [`SpanContainer::span_ident`]: crate::common::SpanContainer::span_ident"] macro_rules ! try_merge_hashset { ($ field : ident : $ self : ident , $ another : ident => $ span : ident) => { { if !$ self .$ field . is_empty () { for ty in $ self .$ field { $ another .$ field . replace (ty) . none_or_else (| dup | crate :: common :: parse :: attr :: err :: dup_arg (& dup .$ span ())) ?; } } $ another .$ field } } ; ($ field : ident : $ self : ident , $ another : ident) => { try_merge_hashset ! ($ field : $ self , $ another => span_ident) } ; }
};
}
