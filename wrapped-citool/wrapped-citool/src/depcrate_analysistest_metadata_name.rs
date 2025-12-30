// Generated macro for test_metadata_name (function)
macro_rules! Depcrate_analysistest_metadata_name {
() => {
// Module: crate::analysis
// Provides: {"test_metadata_name"}
// Dependencies: {}
fn test_metadata_name (metadata : & TestSuiteMetadata) -> String { match metadata { TestSuiteMetadata :: CargoPackage { crates , stage , .. } => { format ! ("{} (stage {stage})" , crates . join (", ")) } TestSuiteMetadata :: Compiletest { suite , stage , .. } => { format ! ("{suite} (stage {stage})") } } }
};
}
