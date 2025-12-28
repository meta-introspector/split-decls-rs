macro_rules! deps {
    () => {
        TryReduceWithConsumer!();
    };
}

macro_rules! impl_926 {
    () => {
        deps!();
        impl < 'r , R > Copy for TryReduceWithConsumer < 'r , R > { }
    };
}

impl_926!();