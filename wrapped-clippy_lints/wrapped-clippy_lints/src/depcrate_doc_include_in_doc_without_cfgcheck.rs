// Generated macro for check (function)
macro_rules! Depcrate_doc_include_in_doc_without_cfgcheck {
() => {
// Module: crate::doc::include_in_doc_without_cfg
// Provides: {"check"}
// Dependencies: {}
pub fn check (cx : & EarlyContext < '_ > , attrs : & [Attribute]) { for attr in attrs { if ! attr . span . from_expansion () && let AttrKind :: Normal (ref item) = attr . kind && attr . doc_str () . is_some () && let AttrArgs :: Eq { expr : meta , .. } = & item . item . args && ! attr . span . contains (meta . span) && let Some (snippet) = snippet_opt (cx , attr . span) && let Some (start) = snippet . find ('[') && let Some (end) = snippet . rfind (']') && let snippet = & snippet [start + 1 .. end] && let Some (sub_snippet) = snippet . trim () . strip_prefix ("doc") && let Some (sub_snippet) = sub_snippet . trim () . strip_prefix ("=") && sub_snippet . trim () . starts_with ("include_str!") { span_lint_and_sugg (cx , DOC_INCLUDE_WITHOUT_CFG , attr . span , "included a file in documentation unconditionally" , "use `cfg_attr(doc, doc = \"...\")`" , format ! ("#{}[cfg_attr(doc, {snippet})]" , if attr . style == AttrStyle :: Inner { "!" } else { "" }) , Applicability :: MachineApplicable ,) ; } } }
};
}
