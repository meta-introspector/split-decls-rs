// Generated macro for lang_item (function)
macro_rules! Depcrate_lang_itemlang_item {
() => {
// Module: crate::lang_item
// Provides: {"lang_item"}
// Dependencies: {}
# [doc = " Salsa query. Look for a lang item, starting from the specified crate and recursively"] # [doc = " traversing its dependencies."] # [salsa_macros :: tracked] pub fn lang_item (db : & dyn DefDatabase , start_crate : Crate , item : LangItem ,) -> Option < LangItemTarget > { let _p = tracing :: info_span ! ("lang_item_query") . entered () ; if let Some (target) = crate_lang_items (db , start_crate) . as_ref () . and_then (| it | it . items . get (& item) . copied ()) { return Some (target) ; } crate_local_def_map (db , start_crate) . local (db) . extern_prelude () . find_map (| (_ , (krate , _)) | { if krate . krate == start_crate { None } else { lang_item (db , krate . krate , item) } }) }
};
}
