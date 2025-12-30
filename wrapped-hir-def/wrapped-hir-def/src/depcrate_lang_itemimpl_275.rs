// Generated macro for impl_275 (impl)
macro_rules! Depcrate_lang_itemimpl_275 {
() => {
// Module: crate::lang_item
// Provides: {"impl_275"}
// Dependencies: {}
impl LangItems { pub fn target (& self , item : LangItem) -> Option < LangItemTarget > { self . items . get (& item) . copied () } fn collect_lang_item < T > (& mut self , db : & dyn DefDatabase , item : T , constructor : fn (T) -> LangItemTarget ,) where T : Into < AttrDefId > + Copy , { let _p = tracing :: info_span ! ("collect_lang_item") . entered () ; if let Some (lang_item) = lang_attr (db , item . into ()) { self . items . entry (lang_item) . or_insert_with (| | constructor (item)) ; } } }
};
}
