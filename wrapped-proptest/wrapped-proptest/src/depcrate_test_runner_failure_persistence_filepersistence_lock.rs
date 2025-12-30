// Generated macro for PERSISTENCE_LOCK (static)
macro_rules! Depcrate_test_runner_failure_persistence_filePERSISTENCE_LOCK {
() => {
// Module: crate::test_runner::failure_persistence::file
// Provides: {"PERSISTENCE_LOCK"}
// Dependencies: {}
# [doc = " Used to guard access to the persistence file(s) so that a single"] # [doc = " process will not step on its own toes."] # [doc = ""] # [doc = " We don't have much protecting us should two separate process try to"] # [doc = " write to the same file at once (depending on how atomic append mode is"] # [doc = " on the OS), but this should be extremely rare."] static PERSISTENCE_LOCK : RwLock < () > = RwLock :: new (()) ;
};
}
