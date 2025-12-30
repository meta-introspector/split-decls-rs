// Generated macro for impl_63 (impl)
macro_rules! Depcrate_defsimpl_63 {
() => {
// Module: crate::defs
// Provides: {"impl_63"}
// Dependencies: {}
impl AsAssocItem for Definition { fn as_assoc_item (self , db : & dyn hir :: db :: HirDatabase) -> Option < AssocItem > { match self { Definition :: Function (it) => it . as_assoc_item (db) , Definition :: Const (it) => it . as_assoc_item (db) , Definition :: TypeAlias (it) => it . as_assoc_item (db) , _ => None , } } }
};
}
