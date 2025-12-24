use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod toolchain_info {
    pub mod rustc_cfg;
    pub mod target_data;
    pub mod target_tuple;
    pub mod version;
    use std::path::Path;
    use crate::{ManifestPath, Sysroot, cargo_config_file::CargoConfigFile};
    #[derive(Copy, Clone)]
    pub enum QueryConfig<'a> {
        /// Directly invoke `rustc` to query the desired information.
        Rustc(&'a Sysroot, &'a Path),
        /// Attempt to use cargo to query the desired information, honoring cargo configurations.
        /// If this fails, falls back to invoking `rustc` directly.
        Cargo(&'a Sysroot, &'a ManifestPath, &'a Option<CargoConfigFile>),
    }
}
