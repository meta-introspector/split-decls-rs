macro_rules! deps {
    () => {
        Cli!();
    };
}

macro_rules! run_analyze_command {
    () => {
        deps!();
        # [cfg (not (feature = "nix_generation"))] pub fn run_analyze_command (_args : & AnalyzeArgs , _cli : & Cli) -> Result < () > { anyhow :: bail ! ("`analyze` command is not available because the `nix_generation` feature is not enabled.") ; }
    };
}

run_analyze_command!()