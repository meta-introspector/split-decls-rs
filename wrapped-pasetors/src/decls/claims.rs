macro_rules! deps {
    () => {
        Claims!();
    };
}

macro_rules! claims {
    () => {
        deps!();
        # [cfg (feature = "std")] # [doc = " Claims for tokens and validation thereof."] pub mod claims ;
    };
}

claims!();