macro_rules! enter {
    () => {
        # [cfg (feature = "std")] mod enter ;
    };
}

enter!();