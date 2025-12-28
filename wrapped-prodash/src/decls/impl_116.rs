macro_rules! deps {
    () => {
        Duration!();
        Step!();
        Throughput!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl Throughput { # [doc = " A convenience method to create a new ThroughPut from `value_change_in_timespan` and `timespan`."] pub fn new (value_change_in_timespan : Step , timespan : std :: time :: Duration) -> Self { Throughput { value_change_in_timespan , timespan , } } }
    };
}

impl_116!();