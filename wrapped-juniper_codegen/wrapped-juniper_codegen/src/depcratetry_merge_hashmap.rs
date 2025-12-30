// Generated macro for try_merge_hashmap (macro)
macro_rules! Depcratetry_merge_hashmap {
() => {
// Module: crate
// Provides: {"try_merge_hashmap"}
// Dependencies: {}
# [doc = " Attempts to merge a [`HashMap`] `$field` of a `$self` struct with the same `$field` of"] # [doc = " `$another` struct. If some [`HashMap`] entries are duplicated, then throws a duplication error"] # [doc = " with a [`Span`] related to the `$another` struct (a later one)."] # [doc = ""] # [doc = " The type of [`Span`] may be explicitly specified as one of the [`SpanContainer`] methods."] # [doc = " By default, [`SpanContainer::span_ident`] is used."] # [doc = ""] # [doc = " [`HashMap`]: std::collections::HashMap"] # [doc = " [`Span`]: proc_macro2::Span"] # [doc = " [`SpanContainer`]: crate::common::SpanContainer"] # [doc = " [`SpanContainer::span_ident`]: crate::common::SpanContainer::span_ident"] macro_rules ! try_merge_hashmap { ($ field : ident : $ self : ident , $ another : ident => $ span : ident) => { { if !$ self .$ field . is_empty () { for (ty , rslvr) in $ self .$ field { $ another .$ field . insert (ty , rslvr) . none_or_else (| dup | crate :: common :: parse :: attr :: err :: dup_arg (& dup .$ span ())) ?; } } $ another .$ field } } ; ($ field : ident : $ self : ident , $ another : ident) => { try_merge_hashmap ! ($ field : $ self , $ another => span_ident) } ; }
};
}
