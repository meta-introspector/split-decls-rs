// Generated macro for impl_35 (impl)
macro_rules! Depcrate_local_poolimpl_35 {
() => {
// Module: crate::local_pool
// Provides: {"impl_35"}
// Dependencies: {}
impl Spawn for LocalSpawner { fn spawn_obj (& self , future : FutureObj < 'static , () >) -> Result < () , SpawnError > { if let Some (incoming) = self . incoming . upgrade () { incoming . borrow_mut () . push (future . into ()) ; Ok (()) } else { Err (SpawnError :: shutdown ()) } } fn status (& self) -> Result < () , SpawnError > { if self . incoming . upgrade () . is_some () { Ok (()) } else { Err (SpawnError :: shutdown ()) } } }
};
}
