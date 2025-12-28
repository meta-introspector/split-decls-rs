macro_rules! deps {
    () => {
        RuntimeName!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl windows_core :: RuntimeName for IAgileReference { }
    };
}

impl_51!();