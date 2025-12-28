macro_rules! deps {
    () => {
        Cli!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        # [cfg (not (feature = "clap_enabled"))] impl Cli { pub fn parse () -> Self { Cli { command : None , verbose : false , quiet : false , color : false , dry_run : false , } } }
    };
}

impl_66!();