macro_rules! deps {
    () => {
        CollectionAllocErr!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl core :: error :: Error for CollectionAllocErr { }
    };
}

impl_5!()