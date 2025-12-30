// Generated macro for impl_15 (impl)
macro_rules! Depcrate_spawnimpl_15 {
() => {
// Module: crate::spawn
// Provides: {"impl_15"}
// Dependencies: {}
impl < Sp : ? Sized + LocalSpawn > LocalSpawn for & Sp { fn spawn_local_obj (& self , future : LocalFutureObj < 'static , () >) -> Result < () , SpawnError > { Sp :: spawn_local_obj (self , future) } fn status_local (& self) -> Result < () , SpawnError > { Sp :: status_local (self) } }
};
}
