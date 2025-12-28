macro_rules! deps {
    () => {
        Commands!();
    };
}

macro_rules! Cli {
    () => {
        deps!();
        # [cfg (not (feature = "clap_enabled"))] # [derive (Debug)] pub struct Cli { pub command : Option < Commands > , pub verbose : bool , pub quiet : bool , pub color : bool , pub dry_run : bool , }
    };
}

Cli!()