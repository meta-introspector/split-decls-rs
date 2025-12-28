macro_rules! deps {
    () => {
        Cli!();
    };
}

macro_rules! run_generate_patches_command {
    () => {
        deps!();
        # [cfg (not (feature = "nix_generation"))] pub fn run_generate_patches_command (_args : & GeneratePatchesArgs , _cli : & Cli) -> Result < () > { anyhow :: bail ! ("`generate-patches` command is not available because the `nix_generation` feature is not enabled.") ; }
    };
}

run_generate_patches_command!()