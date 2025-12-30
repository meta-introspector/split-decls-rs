// Generated macro for load_manifest (function)
macro_rules! Depcrateload_manifest {
() => {
// Module: crate
// Provides: {"load_manifest"}
// Dependencies: {}
fn load_manifest (path : & str) -> Value { let contents = fs :: read_to_string (path) . unwrap_or_else (| err | panic ! ("error occurred reading Cargo manifest {path}: {err}")) ; toml :: from_str (& contents) . unwrap_or_else (| err | panic ! ("error occurred parsing Cargo manifest {path}: {err}")) }
};
}
