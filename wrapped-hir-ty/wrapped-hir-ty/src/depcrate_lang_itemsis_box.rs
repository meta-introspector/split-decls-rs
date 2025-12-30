// Generated macro for is_box (function)
macro_rules! Depcrate_lang_itemsis_box {
() => {
// Module: crate::lang_items
// Provides: {"is_box"}
// Dependencies: {}
pub fn is_box (db : & dyn HirDatabase , adt : AdtId) -> bool { let AdtId :: StructId (id) = adt else { return false } ; db . struct_signature (id) . flags . contains (StructFlags :: IS_BOX) }
};
}
