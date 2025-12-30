// Generated macro for MapFailurePersistence (struct)
macro_rules! Depcrate_test_runner_failure_persistence_mapMapFailurePersistence {
() => {
// Module: crate::test_runner::failure_persistence::map
// Provides: {"MapFailurePersistence"}
// Dependencies: {}
# [doc = " Failure persistence option that loads and saves seeds in memory"] # [doc = " on the heap. This may be useful when accumulating test failures"] # [doc = " across multiple `TestRunner` instances for external reporting"] # [doc = " or batched persistence."] # [derive (Clone , Debug , Default , PartialEq)] pub struct MapFailurePersistence { # [doc = " Backing map, keyed by source_file."] pub map : BTreeMap < & 'static str , BTreeSet < PersistedSeed > > , }
};
}
