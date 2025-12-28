macro_rules! deps {
    () => {
        TraceLevel!();
    };
}

macro_rules! TracingCb {
    () => {
        deps!();
        # [doc = " Callback type used to pass tracing events to the subscriber."] # [doc = " see `trace_set` to register a subscriber."] pub type TracingCb = fn (TraceLevel , & [u8]) ;
    };
}

TracingCb!();