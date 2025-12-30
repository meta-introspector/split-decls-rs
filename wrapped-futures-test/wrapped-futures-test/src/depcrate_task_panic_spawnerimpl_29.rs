// Generated macro for impl_29 (impl)
macro_rules! Depcrate_task_panic_spawnerimpl_29 {
() => {
// Module: crate::task::panic_spawner
// Provides: {"impl_29"}
// Dependencies: {}
impl Spawn for PanicSpawner { fn spawn_obj (& self , _future : FutureObj < 'static , () >) -> Result < () , SpawnError > { panic ! ("should not spawn") } }
};
}
