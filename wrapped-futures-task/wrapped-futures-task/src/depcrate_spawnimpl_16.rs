// Generated macro for impl_16 (impl)
macro_rules! Depcrate_spawnimpl_16 {
() => {
// Module: crate::spawn
// Provides: {"impl_16"}
// Dependencies: {}
impl < Sp : ? Sized + LocalSpawn > LocalSpawn for & mut Sp { fn spawn_local_obj (& self , future : LocalFutureObj < 'static , () >) -> Result < () , SpawnError > { Sp :: spawn_local_obj (self , future) } fn status_local (& self) -> Result < () , SpawnError > { Sp :: status_local (self) } }
};
}
