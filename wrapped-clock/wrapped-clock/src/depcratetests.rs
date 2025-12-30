// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_clone () { let clock = Clock { slot : 1 , epoch_start_timestamp : 2 , epoch : 3 , leader_schedule_epoch : 4 , unix_timestamp : 5 , } ; let cloned_clock = clock . clone () ; assert_eq ! (cloned_clock , clock) ; } }
};
}
