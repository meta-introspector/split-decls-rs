// Generated macro for tests (module)
macro_rules! Depcrate_test_runner_failure_persistence_maptests {
() => {
// Module: crate::test_runner::failure_persistence::map
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: test_runner :: failure_persistence :: tests :: * ; # [test] fn initial_map_is_empty () { assert ! (MapFailurePersistence :: default () . load_persisted_failures2 (HI_PATH) . is_empty ()) } # [test] fn seeds_recoverable () { let mut p = MapFailurePersistence :: default () ; p . save_persisted_failure2 (HI_PATH , INC_SEED , & "") ; let restored = p . load_persisted_failures2 (HI_PATH) ; assert_eq ! (1 , restored . len ()) ; assert_eq ! (INC_SEED , * restored . first () . unwrap ()) ; assert ! (p . load_persisted_failures2 (None) . is_empty ()) ; assert ! (p . load_persisted_failures2 (UNREL_PATH) . is_empty ()) ; } # [test] fn seeds_deduplicated () { let mut p = MapFailurePersistence :: default () ; p . save_persisted_failure2 (HI_PATH , INC_SEED , & "") ; p . save_persisted_failure2 (HI_PATH , INC_SEED , & "") ; let restored = p . load_persisted_failures2 (HI_PATH) ; assert_eq ! (1 , restored . len ()) ; } }
};
}
