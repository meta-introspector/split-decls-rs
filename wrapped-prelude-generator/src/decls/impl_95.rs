macro_rules! deps {
    () => {
        FunctionMetrics!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl FunctionMetrics { pub fn new () -> Self { FunctionMetrics { start_time : Instant :: now () , end_time : None , duration_micros : None , call_count : 0 , } } }
    };
}

impl_95!()