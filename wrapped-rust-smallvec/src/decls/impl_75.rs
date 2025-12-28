macro_rules! deps {
    () => {
        CollectionAllocErr!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl core :: error :: Error for CollectionAllocErr { }
    };
}

impl_75!();