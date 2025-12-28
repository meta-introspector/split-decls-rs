macro_rules! deps {
    () => {
        TryReduceConsumer!();
    };
}

macro_rules! impl_916 {
    () => {
        deps!();
        impl < 'r , R , ID > Copy for TryReduceConsumer < 'r , R , ID > { }
    };
}

impl_916!();