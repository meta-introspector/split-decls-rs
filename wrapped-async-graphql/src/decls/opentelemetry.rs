macro_rules! opentelemetry {
    () => {
        # [cfg (feature = "opentelemetry")] mod opentelemetry ;
    };
}

opentelemetry!();