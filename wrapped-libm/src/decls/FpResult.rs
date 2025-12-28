macro_rules! deps {
    () => {
        Status!();
    };
}

macro_rules! FpResult {
    () => {
        deps!();
        # [doc = " A value combined with a floating point status."] pub struct FpResult < T > { pub val : T , # [cfg_attr (not (feature = "unstable-public-internals") , allow (dead_code))] pub status : Status , }
    };
}

FpResult!()