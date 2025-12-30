// Generated macro for get_metadata (function)
macro_rules! Depcrate_cargo_metadataget_metadata {
() => {
// Module: crate::cargo_metadata
// Provides: {"get_metadata"}
// Dependencies: {}
# [doc = " Use `cargo metadata` to get a list of dependencies and their license data."] # [doc = ""] # [doc = " Any dependency with a path beginning with `root_path` is ignored, as we"] # [doc = " assume `reuse` has covered it already."] pub fn get_metadata (cargo : & Path , cargo_home_path : & Path , root_path : & Path , manifest_paths : & [PathBuf] ,) -> Result < BTreeMap < Package , PackageMetadata > , Error > { let mut output = BTreeMap :: new () ; for manifest_path in manifest_paths { if manifest_path . file_name () != Some (OsStr :: new ("Cargo.toml")) { panic ! ("cargo_manifest::get requires a path to a Cargo.toml file") ; } let metadata = cargo_metadata :: MetadataCommand :: new () . cargo_path (cargo) . env ("RUSTC_BOOTSTRAP" , "1") . manifest_path (manifest_path) . exec () ? ; for package in metadata . packages { let package_manifest_path = package . manifest_path . as_path () ; if package_manifest_path . starts_with (root_path) && ! package_manifest_path . starts_with (cargo_home_path) { continue ; } let package_id = Package { name : package . name . to_string () , version : package . version . to_string () } ; output . insert (package_id , PackageMetadata { license : package . license . unwrap_or_else (| | String :: from ("Unspecified")) , authors : package . authors , notices : BTreeMap :: new () , is_in_libstd : None , } ,) ; } } Ok (output) }
};
}
