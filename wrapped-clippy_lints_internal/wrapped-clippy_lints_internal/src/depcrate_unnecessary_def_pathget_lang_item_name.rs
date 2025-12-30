// Generated macro for get_lang_item_name (function)
macro_rules! Depcrate_unnecessary_def_pathget_lang_item_name {
() => {
// Module: crate::unnecessary_def_path
// Provides: {"get_lang_item_name"}
// Dependencies: {}
fn get_lang_item_name (cx : & LateContext < '_ > , def_id : DefId) -> Option < & 'static str > { if let Some ((lang_item , _)) = cx . tcx . lang_items () . iter () . find (| (_ , id) | * id == def_id) { Some (lang_item . variant_name ()) } else { None } }
};
}
