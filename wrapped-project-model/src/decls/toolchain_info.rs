macro_rules! deps {
    () => {
        Sysroot!();
        CargoConfigFile!();
        ManifestPath!();
    };
}

macro_rules! toolchain_info {
    () => {
        deps!();
        pub mod toolchain_info { pub mod rustc_cfg ; pub mod target_data ; pub mod target_tuple ; pub mod version ; use std :: path :: Path ; use crate :: { ManifestPath , Sysroot , cargo_config_file :: CargoConfigFile } ; # [derive (Copy , Clone)] pub enum QueryConfig < 'a > { # [doc = " Directly invoke `rustc` to query the desired information."] Rustc (& 'a Sysroot , & 'a Path) , # [doc = " Attempt to use cargo to query the desired information, honoring cargo configurations."] # [doc = " If this fails, falls back to invoking `rustc` directly."] Cargo (& 'a Sysroot , & 'a ManifestPath , & 'a Option < CargoConfigFile >) , } }
    };
}

toolchain_info!()