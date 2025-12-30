// Generated macro for write_manifest (function)
macro_rules! Depcrate_manifestwrite_manifest {
() => {
// Module: crate::manifest
// Provides: {"write_manifest"}
// Dependencies: {}
# [doc = " Write a Cargo.toml file"] pub fn write_manifest (path : & Path , manifest : & Value) -> Result < () > { let content = toml :: to_string_pretty (manifest) ? ; std :: fs :: write (path , content) ? ; Ok (()) }
};
}
