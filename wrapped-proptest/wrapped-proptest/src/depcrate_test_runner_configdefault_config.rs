// Generated macro for DEFAULT_CONFIG (static)
macro_rules! Depcrate_test_runner_configDEFAULT_CONFIG {
() => {
// Module: crate::test_runner::config
// Provides: {"DEFAULT_CONFIG"}
// Dependencies: {}
# [cfg (feature = "std")] static DEFAULT_CONFIG : std :: sync :: LazyLock < Config > = std :: sync :: LazyLock :: new (| | { let mut default_config = default_default_config () ; default_config . failure_persistence = Some (Box :: new (crate :: test_runner :: FileFailurePersistence :: default ())) ; contextualize_config (default_config) }) ;
};
}
