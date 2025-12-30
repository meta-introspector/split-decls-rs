// Generated macro for impl_54 (impl)
macro_rules! Depcrate_util_flat_csvimpl_54 {
() => {
// Module: crate::util::flat_csv
// Provides: {"impl_54"}
// Dependencies: {}
impl < Sep : Separator > FlatCsv < Sep > { pub (crate) fn iter (& self) -> impl Iterator < Item = & str > { self . value . to_str () . ok () . into_iter () . flat_map (| value_str | { let mut in_quotes = false ; value_str . split (move | c | { # [allow (clippy :: collapsible_else_if)] if in_quotes { if c == '"' { in_quotes = false ; } false } else { if c == Sep :: CHAR { true } else { if c == '"' { in_quotes = true ; } false } } }) . map (| item | item . trim ()) }) } }
};
}
