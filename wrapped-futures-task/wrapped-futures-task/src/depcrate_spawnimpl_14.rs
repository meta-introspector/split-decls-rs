// Generated macro for impl_14 (impl)
macro_rules! Depcrate_spawnimpl_14 {
() => {
// Module: crate::spawn
// Provides: {"impl_14"}
// Dependencies: {}
impl < Sp : ? Sized + Spawn > Spawn for & mut Sp { fn spawn_obj (& self , future : FutureObj < 'static , () >) -> Result < () , SpawnError > { Sp :: spawn_obj (self , future) } fn status (& self) -> Result < () , SpawnError > { Sp :: status (self) } }
};
}
