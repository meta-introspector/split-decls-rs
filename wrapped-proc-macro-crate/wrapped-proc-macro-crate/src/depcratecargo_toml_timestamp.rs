// Generated macro for cargo_toml_timestamp (function)
macro_rules! Depcratecargo_toml_timestamp {
() => {
// Module: crate
// Provides: {"cargo_toml_timestamp"}
// Dependencies: {}
fn cargo_toml_timestamp (manifest_path : & Path) -> Result < SystemTime , Error > { fs :: metadata (manifest_path) . and_then (| meta | meta . modified ()) . map_err (| source | { if source . kind () == io :: ErrorKind :: NotFound { Error :: NotFound (manifest_path . to_owned ()) } else { Error :: CouldNotRead { path : manifest_path . to_owned () , source } } }) }
};
}
