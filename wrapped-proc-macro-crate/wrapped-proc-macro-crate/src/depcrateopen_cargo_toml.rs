// Generated macro for open_cargo_toml (function)
macro_rules! Depcrateopen_cargo_toml {
() => {
// Module: crate
// Provides: {"open_cargo_toml"}
// Dependencies: {}
# [doc = " Open the given `Cargo.toml` and parse it into a hashmap."] fn open_cargo_toml (path : & Path) -> Result < DocumentMut , Error > { let content = fs :: read_to_string (path) . map_err (| e | Error :: CouldNotRead { source : e , path : path . into () }) ? ; content . parse :: < DocumentMut > () . map_err (| e | Error :: InvalidToml { source : e }) }
};
}
