// Generated macro for module_for_assoc_item_loc (function)
macro_rules! Depcratemodule_for_assoc_item_loc {
() => {
// Module: crate
// Provides: {"module_for_assoc_item_loc"}
// Dependencies: {}
# [inline] fn module_for_assoc_item_loc < 'db > (db : & (dyn 'db + DefDatabase) , id : impl Lookup < Database = dyn DefDatabase , Data = AssocItemLoc < impl AstIdNode > > ,) -> ModuleId { id . lookup (db) . container . module (db) }
};
}
