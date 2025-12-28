macro_rules! archive {
    () => {
        # [cfg (feature = "archive")] pub mod archive ;
    };
}

archive!()