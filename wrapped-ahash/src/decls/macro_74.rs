macro_rules! deps {
    () => {
        RandomSource!();
    };
}

macro_rules! macro_74 {
    () => {
        deps!();
        cfg_if :: cfg_if ! { if # [cfg (not (all (target_arch = "arm" , target_os = "none")))] { use once_cell :: race :: OnceBox ; static RAND_SOURCE : OnceBox < Box < dyn RandomSource + Send + Sync >> = OnceBox :: new () ; } }
    };
}

macro_74!();