macro_rules! claims {
    () => {
        # [cfg (feature = "std")] # [doc = " Claims for tokens and validation thereof."] pub mod claims ;
    };
}

claims!()