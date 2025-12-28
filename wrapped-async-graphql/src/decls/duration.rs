macro_rules! duration {
    () => {
        # [cfg (feature = "chrono-duration")] mod duration ;
    };
}

duration!();