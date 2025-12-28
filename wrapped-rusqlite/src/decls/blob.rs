macro_rules! blob {
    () => {
        # [cfg (feature = "blob")] pub mod blob ;
    };
}

blob!()