// Generated macro for tests (module)
macro_rules! Depcrate_thread_pooltests {
() => {
// Module: crate::thread_pool
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_drop_after_start () { { let (tx , rx) = mpsc :: sync_channel (2) ; let _cpu_pool = ThreadPoolBuilder :: new () . pool_size (2) . after_start (move | _ | tx . send (1) . unwrap ()) . create () . unwrap () ; let count = rx . into_iter () . count () ; assert_eq ! (count , 2) ; } std :: thread :: sleep (std :: time :: Duration :: from_millis (500)) ; } }
};
}
