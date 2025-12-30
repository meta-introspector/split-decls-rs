// Generated macro for impl_433 (impl)
macro_rules! Depcrateimpl_433 {
() => {
// Module: crate
// Provides: {"impl_433"}
// Dependencies: {}
impl ExternAssocItem { pub fn name (self , db : & dyn HirDatabase) -> Name { match self { Self :: Function (it) => it . name (db) , Self :: Static (it) => it . name (db) , Self :: TypeAlias (it) => it . name (db) , } } pub fn module (self , db : & dyn HirDatabase) -> Module { match self { Self :: Function (f) => f . module (db) , Self :: Static (c) => c . module (db) , Self :: TypeAlias (t) => t . module (db) , } } pub fn as_function (self) -> Option < Function > { match self { Self :: Function (v) => Some (v) , _ => None , } } pub fn as_static (self) -> Option < Static > { match self { Self :: Static (v) => Some (v) , _ => None , } } pub fn as_type_alias (self) -> Option < TypeAlias > { match self { Self :: TypeAlias (v) => Some (v) , _ => None , } } }
};
}
