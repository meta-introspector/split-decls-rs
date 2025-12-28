macro_rules! tracing {
    () => {
        # [cfg (feature = "tracing")] mod tracing ;
    };
}

tracing!()