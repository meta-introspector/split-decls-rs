// Generated macro for check (function)
macro_rules! Depcrate_attrs_allow_attributes_without_reasoncheck {
() => {
// Module: crate::attrs::allow_attributes_without_reason
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'cx > (cx : & EarlyContext < 'cx > , name : Symbol , items : & [MetaItemInner] , attr : & 'cx Attribute) { if let Some (item) = items . last () . and_then (MetaItemInner :: meta_item) && let MetaItemKind :: NameValue (_) = & item . kind && item . path == sym :: reason { return ; } if attr . span . in_external_macro (cx . sess () . source_map ()) || is_from_proc_macro (cx , attr) { return ; } # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , ALLOW_ATTRIBUTES_WITHOUT_REASON , attr . span , format ! ("`{name}` attribute without specifying a reason") , | diag | { diag . help ("try adding a reason at the end with `, reason = \"..\"`") ; } ,) ; }
};
}
