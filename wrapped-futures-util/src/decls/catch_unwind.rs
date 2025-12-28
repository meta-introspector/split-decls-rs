macro_rules! catch_unwind {
    () => {
        # [cfg (feature = "std")] mod catch_unwind ;
    };
}

catch_unwind!()