// Generated macro for impl_36 (impl)
macro_rules! Depcrate_local_poolimpl_36 {
() => {
// Module: crate::local_pool
// Provides: {"impl_36"}
// Dependencies: {}
impl LocalSpawn for LocalSpawner { fn spawn_local_obj (& self , future : LocalFutureObj < 'static , () >) -> Result < () , SpawnError > { if let Some (incoming) = self . incoming . upgrade () { incoming . borrow_mut () . push (future) ; Ok (()) } else { Err (SpawnError :: shutdown ()) } } fn status_local (& self) -> Result < () , SpawnError > { if self . incoming . upgrade () . is_some () { Ok (()) } else { Err (SpawnError :: shutdown ()) } } }
};
}
