macro_rules! err {
    () => {
        # [cfg (feature = "std")] mod err ;
    };
}

err!();