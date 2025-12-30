// Generated macro for test_output_types_tracking_hash_different_construction_order (function)
macro_rules! Depcrate_teststest_output_types_tracking_hash_different_construction_order {
() => {
// Module: crate::tests
// Provides: {"test_output_types_tracking_hash_different_construction_order"}
// Dependencies: {}
# [test] fn test_output_types_tracking_hash_different_construction_order () { let mut v1 = Options :: default () ; let mut v2 = Options :: default () ; v1 . output_types = OutputTypes :: new (& [(OutputType :: Exe , Some (OutFileName :: Real (PathBuf :: from ("./some/thing")))) , (OutputType :: Bitcode , Some (OutFileName :: Real (PathBuf :: from ("./some/thing.bc")))) ,]) ; v2 . output_types = OutputTypes :: new (& [(OutputType :: Bitcode , Some (OutFileName :: Real (PathBuf :: from ("./some/thing.bc")))) , (OutputType :: Exe , Some (OutFileName :: Real (PathBuf :: from ("./some/thing")))) ,]) ; assert_same_hash (& v1 , & v2) ; }
};
}
