macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < Fut > FusedStream for Flatten < Fut , Fut :: Output > where Fut : Future , Fut :: Output : Stream , { fn is_terminated (& self) -> bool { matches ! (self , Self :: Empty) } }
    };
}

impl_42!();