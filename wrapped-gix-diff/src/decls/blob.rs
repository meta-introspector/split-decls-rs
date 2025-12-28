macro_rules! blob {
    () => {
        # [doc = ""] # [cfg (feature = "blob")] pub mod blob ;
    };
}

blob!();