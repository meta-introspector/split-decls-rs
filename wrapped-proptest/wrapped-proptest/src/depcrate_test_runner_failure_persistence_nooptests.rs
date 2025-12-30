// Generated macro for tests (module)
macro_rules! Depcrate_test_runner_failure_persistence_nooptests {
() => {
// Module: crate::test_runner::failure_persistence::noop
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: test_runner :: failure_persistence :: tests :: * ; # [test] fn default_load_is_empty () { assert ! (NoopFailurePersistence :: default () . load_persisted_failures2 (None) . is_empty ()) ; assert ! (NoopFailurePersistence :: default () . load_persisted_failures2 (HI_PATH) . is_empty ()) ; } # [test] fn seeds_not_recoverable () { let mut p = NoopFailurePersistence :: default () ; p . save_persisted_failure2 (HI_PATH , INC_SEED , & "") ; assert ! (p . load_persisted_failures2 (HI_PATH) . is_empty ()) ; assert ! (p . load_persisted_failures2 (None) . is_empty ()) ; assert ! (p . load_persisted_failures2 (UNREL_PATH) . is_empty ()) ; } }
};
}
