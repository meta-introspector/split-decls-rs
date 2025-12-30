// Generated macro for as_extern_assoc_item (function)
macro_rules! Depcrateas_extern_assoc_item {
() => {
// Module: crate
// Provides: {"as_extern_assoc_item"}
// Dependencies: {}
fn as_extern_assoc_item < 'db , ID , DEF , LOC > (db : & (dyn HirDatabase + 'db) , ctor : impl FnOnce (DEF) -> ExternAssocItem , id : ID ,) -> Option < ExternAssocItem > where ID : Lookup < Database = dyn DefDatabase , Data = AssocItemLoc < LOC > > , DEF : From < ID > , LOC : AstIdNode , { match id . lookup (db) . container { ItemContainerId :: ExternBlockId (_) => Some (ctor (DEF :: from (id))) , ItemContainerId :: TraitId (_) | ItemContainerId :: ImplId (_) | ItemContainerId :: ModuleId (_) => { None } } }
};
}
