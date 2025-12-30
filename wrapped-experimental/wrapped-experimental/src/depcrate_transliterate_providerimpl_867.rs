// Generated macro for impl_867 (impl)
macro_rules! Depcrate_transliterate_providerimpl_867 {
() => {
// Module: crate::transliterate::provider
// Provides: {"impl_867"}
// Dependencies: {}
impl RuleBasedTransliterator < '_ > { # [doc = " Returns an iterator of dependencies on other transliterators."] # [doc = ""] # [doc = " Note that this may contain duplicate entries."] pub fn deps (& self) -> impl Iterator < Item = Cow < '_ , str > > { use zerofrom :: ZeroFrom ; self . id_group_list . iter () . flat_map (| id_group | id_group . iter () . map (| s | SimpleId :: zero_from (s) . id)) . chain (self . variable_table . function_calls . iter () . map (| s | FunctionCall :: zero_from (s) . translit . id) ,) } }
};
}
