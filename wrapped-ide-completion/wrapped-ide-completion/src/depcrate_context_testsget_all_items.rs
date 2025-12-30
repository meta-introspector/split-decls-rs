// Generated macro for get_all_items (function)
macro_rules! Depcrate_context_testsget_all_items {
() => {
// Module: crate::context::tests
// Provides: {"get_all_items"}
// Dependencies: {}
pub (crate) fn get_all_items (config : CompletionConfig < '_ > , code : & str , trigger_character : Option < char > ,) -> Vec < CompletionItem > { let (db , position) = position (code) ; let res = hir :: attach_db (& db , | | { HirDatabase :: zalsa_register_downcaster (& db) ; crate :: completions (& db , & config , position , trigger_character) }) . map_or_else (Vec :: default , Into :: into) ; res . iter () . for_each (| it | { let sr = it . source_range ; assert ! (sr . contains_inclusive (position . offset) , "source range {sr:?} does not contain the offset {:?} of the completion request: {it:?}" , position . offset) ; }) ; res }
};
}
