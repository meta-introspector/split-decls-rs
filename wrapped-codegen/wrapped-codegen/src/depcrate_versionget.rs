// Generated macro for get (function)
macro_rules! Depcrate_versionget {
() => {
// Module: crate::version
// Provides: {"get"}
// Dependencies: {}
pub fn get () -> Result < Version > { let syn_cargo_toml = workspace_path :: get ("Cargo.toml") ; let content = fs :: read_to_string (syn_cargo_toml) ? ; let manifest : Table = toml :: from_str (& content) ? ; manifest . get ("package") . context ("[package] not found in Cargo.toml") ? . get ("version") . and_then (toml :: Value :: as_str) . context ("package version not found in Cargo.toml") ? . parse () . context ("failed to parse package version") }
};
}
