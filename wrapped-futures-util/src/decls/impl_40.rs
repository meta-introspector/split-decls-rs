macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < Fut > FusedFuture for Flatten < Fut , Fut :: Output > where Fut : Future , Fut :: Output : Future , { fn is_terminated (& self) -> bool { matches ! (self , Self :: Empty) } }
    };
}

impl_40!()