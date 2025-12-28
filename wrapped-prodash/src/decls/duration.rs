macro_rules! duration {
    () => {
        # [cfg (feature = "unit-duration")] mod duration ;
    };
}

duration!()