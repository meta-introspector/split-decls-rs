// Split declarations module
// This module includes all the declarations that were split from the original lib.rs

#[path = "decls/split_decls_rs_decls_Package.rs"]
pub mod package;

#[path = "decls/split_decls_rs_decls_CargoToml.rs"] 
pub mod cargo_toml;

#[path = "decls/split_decls_rs_decls_CratePaths.rs"]
pub mod crate_paths;

#[path = "decls/split_decls_rs_decls_process_dependency_table.rs"]
pub mod process_dependency_table;

// Re-export the declarations
pub use package::*;
pub use cargo_toml::*;
pub use crate_paths::*;
pub use process_dependency_table::*;
