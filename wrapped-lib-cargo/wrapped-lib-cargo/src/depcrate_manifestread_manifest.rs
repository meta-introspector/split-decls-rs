// Generated macro for read_manifest (function)
macro_rules! Depcrate_manifestread_manifest {
() => {
// Module: crate::manifest
// Provides: {"read_manifest"}
// Dependencies: {}
# [doc = " Read and parse a Cargo.toml file"] pub fn read_manifest (path : & Path) -> Result < Value > { let content = std :: fs :: read_to_string (path) ? ; let manifest : Value = toml :: from_str (& content) ? ; Ok (manifest) }
};
}
