// Generated macro for FetchMetadata (struct)
macro_rules! Depcrate_cargo_workspaceFetchMetadata {
() => {
// Module: crate::cargo_workspace
// Provides: {"FetchMetadata"}
// Dependencies: {}
pub (crate) struct FetchMetadata { command : cargo_metadata :: MetadataCommand , # [expect (dead_code)] manifest_path : ManifestPath , lockfile_path : Option < Utf8PathBuf > , # [expect (dead_code)] kind : & 'static str , no_deps : bool , no_deps_result : anyhow :: Result < cargo_metadata :: Metadata > , other_options : Vec < String > , }
};
}
