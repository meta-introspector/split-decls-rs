macro_rules! Commands {
    () => {
        # [cfg (not (feature = "clap_enabled"))] # [derive (Debug)] pub enum Commands { }
    };
}

Commands!();