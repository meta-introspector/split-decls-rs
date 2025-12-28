macro_rules! macro_260 {
    () => {
        cfg_if ! { if # [cfg (feature = "unstable-public-internals")] { pub mod generic ; } else { mod generic ; } }
    };
}

macro_260!();