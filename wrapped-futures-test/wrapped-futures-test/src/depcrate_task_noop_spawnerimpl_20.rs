// Generated macro for impl_20 (impl)
macro_rules! Depcrate_task_noop_spawnerimpl_20 {
() => {
// Module: crate::task::noop_spawner
// Provides: {"impl_20"}
// Dependencies: {}
impl Spawn for NoopSpawner { fn spawn_obj (& self , _future : FutureObj < 'static , () >) -> Result < () , SpawnError > { Ok (()) } }
};
}
