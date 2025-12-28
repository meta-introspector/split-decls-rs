macro_rules! deps {
    () => {
        FindConsumer!();
    };
}

macro_rules! impl_482 {
    () => {
        deps!();
        impl < 'p , P > FindConsumer < 'p , P > { fn new (find_op : & 'p P , found : & 'p AtomicBool) -> Self { FindConsumer { find_op , found } } }
    };
}

impl_482!();