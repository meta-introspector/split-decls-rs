// Generated macro for load_warnings (function)
macro_rules! Depcrate_jsonload_warnings {
() => {
// Module: crate::json
// Provides: {"load_warnings"}
// Dependencies: {}
# [doc = " Loads lint warnings from a JSON file at the given path."] fn load_warnings (path : & Path) -> Vec < LintJson > { let file = fs :: read (path) . unwrap_or_else (| e | panic ! ("failed to read {}: {e}" , path . display ())) ; serde_json :: from_slice (& file) . unwrap_or_else (| e | panic ! ("failed to deserialize {}: {e}" , path . display ())) }
};
}
