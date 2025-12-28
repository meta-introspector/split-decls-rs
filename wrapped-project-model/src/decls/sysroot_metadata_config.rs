macro_rules! deps {
    () => {
        CargoConfig!();
        CargoMetadataConfig!();
    };
}

macro_rules! sysroot_metadata_config {
    () => {
        deps!();
        fn sysroot_metadata_config (config : & CargoConfig , targets : & [String] , toolchain_version : Option < Version > ,) -> CargoMetadataConfig { CargoMetadataConfig { features : Default :: default () , targets : targets . to_vec () , extra_args : Default :: default () , extra_env : config . extra_env . clone () , toolchain_version , kind : "sysroot" , } }
    };
}

sysroot_metadata_config!()