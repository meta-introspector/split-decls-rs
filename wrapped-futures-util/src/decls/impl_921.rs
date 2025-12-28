macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_921 {
    () => {
        deps!();
        impl < T > Unpin for Drain < T > { }
    };
}

impl_921!()