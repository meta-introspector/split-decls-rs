// Generated macro for build (function)
macro_rules! Depcrate_core_metadatabuild {
() => {
// Module: crate::core::metadata
// Provides: {"build"}
// Dependencies: {}
# [doc = " Collects and stores package metadata of each workspace members into `build`,"] # [doc = " by executing `cargo metadata` commands."] pub fn build (build : & mut Build) { for package in workspace_members (build) { if package . source . is_none () { let name = package . name ; let mut path = PathBuf :: from (package . manifest_path) ; path . pop () ; let deps = package . dependencies . into_iter () . filter (| dep | dep . source . is_none ()) . map (| dep | dep . name) . collect () ; let krate = Crate { name : name . clone () , deps , path , features : package . features . keys () . cloned () . collect () , } ; let relative_path = krate . local_path (build) ; build . crates . insert (name . clone () , krate) ; let existing_path = build . crate_paths . insert (relative_path , name) ; assert ! (existing_path . is_none () , "multiple crates with the same path: {}" , existing_path . unwrap ()) ; } } }
};
}
