macro_rules! footer {
    () => {
        # [cfg (feature = "std")] # [doc = " Footer for tokens."] pub mod footer ;
    };
}

footer!()