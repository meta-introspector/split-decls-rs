macro_rules! macro_30 {
    () => {
        cfg_if :: cfg_if ! { if # [cfg (feature = "std")] { use std :: borrow :: Cow ; use std :: fmt ; use std :: path :: PathBuf ; use std :: prelude :: v1 ::*; use std :: str ; } }
    };
}

macro_30!();