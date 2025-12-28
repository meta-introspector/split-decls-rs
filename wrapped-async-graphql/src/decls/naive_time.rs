macro_rules! naive_time {
    () => {
        # [cfg (feature = "chrono")] mod naive_time ;
    };
}

naive_time!()