// Generated macro for test_deserialization (function)
macro_rules! Depcrate_discovertest_deserialization {
() => {
// Module: crate::discover
// Provides: {"test_deserialization"}
// Dependencies: {}
# [test] fn test_deserialization () { let message = r#"
    {"kind": "progress", "message":"querying build system","input":{"files":["src/main.rs"]}}
    "# ; let message : DiscoverProjectData = serde_json :: from_str (message) . expect ("Unable to deserialize message") ; assert ! (matches ! (message , DiscoverProjectData :: Progress { .. })) ; let message = r#"
    {"kind": "error", "error":"failed to deserialize command output","source":"command"}
    "# ; let message : DiscoverProjectData = serde_json :: from_str (message) . expect ("Unable to deserialize message") ; assert ! (matches ! (message , DiscoverProjectData :: Error { .. })) ; let message = r#"
    {"kind": "finished", "project": {"sysroot": "foo", "crates": [], "runnables": []}, "buildfile":"rust-analyzer/BUILD"}
    "# ; let message : DiscoverProjectData = serde_json :: from_str (message) . expect ("Unable to deserialize message") ; assert ! (matches ! (message , DiscoverProjectData :: Finished { .. })) ; }
};
}
