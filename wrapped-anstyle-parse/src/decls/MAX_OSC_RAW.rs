macro_rules! MAX_OSC_RAW {
    () => {
        # [cfg (feature = "core")] const MAX_OSC_RAW : usize = 1024 ;
    };
}

MAX_OSC_RAW!();