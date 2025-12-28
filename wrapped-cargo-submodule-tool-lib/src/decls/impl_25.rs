macro_rules! deps {
    () => {
        NonVendoredModuleFinder!();
        RealNonVendoredModuleFinder!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        # [cfg (not (feature = "nix_generation"))] impl NonVendoredModuleFinder for RealNonVendoredModuleFinder { fn find_and_count_non_vendored (& self , _tree_file_path : & Path , _project_root : & Path ,) -> Result < HashMap < String , u32 > > { anyhow :: bail ! ("`NonVendoredModuleFinder` requires the `nix_generation` feature to be enabled.") ; } }
    };
}

impl_25!();