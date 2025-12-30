// Generated macro for impl_64 (impl)
macro_rules! Depcrate_defsimpl_64 {
() => {
// Module: crate::defs
// Provides: {"impl_64"}
// Dependencies: {}
impl AsExternAssocItem for Definition { fn as_extern_assoc_item (self , db : & dyn hir :: db :: HirDatabase) -> Option < ExternAssocItem > { match self { Definition :: Function (it) => it . as_extern_assoc_item (db) , Definition :: Static (it) => it . as_extern_assoc_item (db) , Definition :: TypeAlias (it) => it . as_extern_assoc_item (db) , _ => None , } } }
};
}
