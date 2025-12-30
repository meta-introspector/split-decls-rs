// Generated macro for impl_49 (impl)
macro_rules! Depcrate_task_record_spawnerimpl_49 {
() => {
// Module: crate::task::record_spawner
// Provides: {"impl_49"}
// Dependencies: {}
impl RecordSpawner { # [doc = " Create a new instance"] pub fn new () -> Self { Default :: default () } # [doc = " Inspect any futures that were spawned onto this [`Spawn`]."] pub fn spawned (& self) -> Ref < '_ , Vec < FutureObj < 'static , () > > > { self . spawned . borrow () } }
};
}
