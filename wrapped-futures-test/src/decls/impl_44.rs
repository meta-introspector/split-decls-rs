macro_rules! deps {
    () => {
        PendingOnce!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < Fut : Future > PendingOnce < Fut > { pub (super) fn new (future : Fut) -> Self { Self { future , polled_before : false } } }
    };
}

impl_44!();