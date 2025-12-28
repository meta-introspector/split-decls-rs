macro_rules! macro_10 {
    () => {
        cfg_if :: cfg_if ! { if # [cfg (feature = "std")] { use std :: path :: Path ; use std :: prelude :: v1 ::*; } }
    };
}

macro_10!()