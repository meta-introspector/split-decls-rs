// Generated macro for impl_863 (impl)
macro_rules! Depcrateimpl_863 {
() => {
// Module: crate
// Provides: {"impl_863"}
// Dependencies: {}
impl HasModule for CrateRootModuleId { # [inline] fn module (& self , _db : & dyn DefDatabase) -> ModuleId { ModuleId { krate : self . krate , block : None , local_id : DefMap :: ROOT } } # [inline] fn krate (& self , _db : & dyn DefDatabase) -> Crate { self . krate } }
};
}
