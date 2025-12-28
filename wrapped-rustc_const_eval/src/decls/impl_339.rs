macro_rules! deps {
    () => {
        EnteredTraceSpan!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl EnteredTraceSpan for tracing :: span :: EnteredSpan { fn or_if_tracing_disabled (self , _f : impl FnOnce ()) -> Self { self } }
    };
}

impl_339!();