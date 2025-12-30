// Generated macro for impl_13 (impl)
macro_rules! Depcrate_spawnimpl_13 {
() => {
// Module: crate::spawn
// Provides: {"impl_13"}
// Dependencies: {}
impl < Sp : ? Sized + Spawn > Spawn for & Sp { fn spawn_obj (& self , future : FutureObj < 'static , () >) -> Result < () , SpawnError > { Sp :: spawn_obj (self , future) } fn status (& self) -> Result < () , SpawnError > { Sp :: status (self) } }
};
}
