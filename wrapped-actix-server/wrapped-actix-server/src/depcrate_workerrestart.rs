// Generated macro for Restart (struct)
macro_rules! Depcrate_workerRestart {
() => {
// Module: crate::worker
// Provides: {"Restart"}
// Dependencies: {}
struct Restart { factory_id : usize , token : usize , fut : LocalBoxFuture < 'static , Result < (usize , BoxedServerService) , () > > , }
};
}
