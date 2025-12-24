use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CrateInfo {
    pub name: &'static str,
    pub path: &'static str,
}

// Include the generated data directly from the OUT_DIR.
// This assumes the build.rs script has successfully generated this file.
// The build.rs of THIS crate (cargo-metadata-lib) will generate this.
include!(concat!(env!("OUT_DIR"), "/cargo_tree_data.rs"));

pub fn get_cargo_tree_data() -> &'static [CrateInfo] {
    &CARGO_TREE_DATA
}