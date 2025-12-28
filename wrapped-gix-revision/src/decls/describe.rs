macro_rules! describe {
    () => {
        # [doc = ""] # [cfg (feature = "describe")] pub mod describe ;
    };
}

describe!();