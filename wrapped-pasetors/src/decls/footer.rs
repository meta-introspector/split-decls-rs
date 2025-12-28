macro_rules! deps {
    () => {
        Footer!();
    };
}

macro_rules! footer {
    () => {
        deps!();
        # [cfg (feature = "std")] # [doc = " Footer for tokens."] pub mod footer ;
    };
}

footer!()