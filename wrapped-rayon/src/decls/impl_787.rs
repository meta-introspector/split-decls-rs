macro_rules! deps {
    () => {
        ReduceConsumer!();
    };
}

macro_rules! impl_787 {
    () => {
        deps!();
        impl < 'r , R , ID > Copy for ReduceConsumer < 'r , R , ID > { }
    };
}

impl_787!();