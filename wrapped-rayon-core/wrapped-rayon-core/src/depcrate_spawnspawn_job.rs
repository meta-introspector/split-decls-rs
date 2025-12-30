// Generated macro for spawn_job (function)
macro_rules! Depcrate_spawnspawn_job {
() => {
// Module: crate::spawn
// Provides: {"spawn_job"}
// Dependencies: {}
unsafe fn spawn_job < F > (func : F , registry : & Arc < Registry >) -> JobRef where F : FnOnce () + Send + 'static , { registry . increment_terminate_count () ; HeapJob :: new ({ let registry = Arc :: clone (registry) ; move | | { registry . catch_unwind (func) ; registry . terminate () ; } }) . into_static_job_ref () }
};
}
