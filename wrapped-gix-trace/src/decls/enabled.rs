macro_rules! enabled {
    () => {
        # [cfg (feature = "tracing")] mod enabled ;
    };
}

enabled!()