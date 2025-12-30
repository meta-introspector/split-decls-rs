// Generated macro for impl_926 (impl)
macro_rules! Depcrateimpl_926 {
() => {
// Module: crate
// Provides: {"impl_926"}
// Dependencies: {}
impl < N , ItemId > HasModule for ItemId where N : AstIdNode , ItemId : Lookup < Database = dyn DefDatabase , Data = ItemLoc < N > > + Copy , { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { self . lookup (db) . container } }
};
}
