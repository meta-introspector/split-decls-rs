macro_rules! deps {
    () => {
        EnteredTraceSpan!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl EnteredTraceSpan for () { fn or_if_tracing_disabled (self , f : impl FnOnce ()) -> Self { f () ; self } }
    };
}

impl_338!()