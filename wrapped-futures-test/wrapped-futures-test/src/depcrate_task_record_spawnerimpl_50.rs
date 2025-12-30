// Generated macro for impl_50 (impl)
macro_rules! Depcrate_task_record_spawnerimpl_50 {
() => {
// Module: crate::task::record_spawner
// Provides: {"impl_50"}
// Dependencies: {}
impl Spawn for RecordSpawner { fn spawn_obj (& self , future : FutureObj < 'static , () >) -> Result < () , SpawnError > { self . spawned . borrow_mut () . push (future) ; Ok (()) } }
};
}
