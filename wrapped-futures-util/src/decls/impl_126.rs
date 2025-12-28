macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < Fut > FusedFuture for TryFlattenErr < Fut , Fut :: Error > where Fut : TryFuture , Fut :: Error : TryFuture < Ok = Fut :: Ok > , { fn is_terminated (& self) -> bool { matches ! (self , Self :: Empty) } }
    };
}

impl_126!()