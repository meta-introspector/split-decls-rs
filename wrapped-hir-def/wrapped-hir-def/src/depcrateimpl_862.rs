// Generated macro for impl_862 (impl)
macro_rules! Depcrateimpl_862 {
() => {
// Module: crate
// Provides: {"impl_862"}
// Dependencies: {}
impl CrateRootModuleId { pub fn def_map (self , db : & dyn DefDatabase) -> & DefMap { crate_def_map (db , self . krate) } pub (crate) fn local_def_map (self , db : & dyn DefDatabase) -> (& DefMap , & LocalDefMap) { let def_map = crate_local_def_map (db , self . krate) ; (def_map . def_map (db) , def_map . local (db)) } pub fn krate (self) -> Crate { self . krate } }
};
}
