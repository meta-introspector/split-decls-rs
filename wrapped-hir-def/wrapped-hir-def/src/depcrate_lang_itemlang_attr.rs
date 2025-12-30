// Generated macro for lang_attr (function)
macro_rules! Depcrate_lang_itemlang_attr {
() => {
// Module: crate::lang_item
// Provides: {"lang_attr"}
// Dependencies: {}
pub (crate) fn lang_attr (db : & dyn DefDatabase , item : AttrDefId) -> Option < LangItem > { db . attrs (item) . lang_item () }
};
}
