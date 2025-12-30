// Generated macro for impl_53 (impl)
macro_rules! Depcrate_configimpl_53 {
() => {
// Module: crate::config
// Provides: {"impl_53"}
// Dependencies: {}
impl CompletionConfig < '_ > { pub fn postfix_snippets (& self) -> impl Iterator < Item = (& str , & Snippet) > { self . snippets . iter () . flat_map (| snip | snip . postfix_triggers . iter () . map (move | trigger | (& * * trigger , snip))) } pub fn prefix_snippets (& self) -> impl Iterator < Item = (& str , & Snippet) > { self . snippets . iter () . flat_map (| snip | snip . prefix_triggers . iter () . map (move | trigger | (& * * trigger , snip))) } pub fn find_path_config (& self , allow_unstable : bool) -> FindPathConfig { FindPathConfig { prefer_no_std : self . prefer_no_std , prefer_prelude : self . prefer_prelude , prefer_absolute : self . prefer_absolute , allow_unstable , } } pub fn import_path_config (& self) -> ImportPathConfig { ImportPathConfig { prefer_no_std : self . prefer_no_std , prefer_prelude : self . prefer_prelude , prefer_absolute : self . prefer_absolute , } } }
};
}
