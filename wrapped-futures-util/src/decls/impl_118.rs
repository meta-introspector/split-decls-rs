macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < Fut > FusedFuture for TryFlatten < Fut , Fut :: Ok > where Fut : TryFuture , Fut :: Ok : TryFuture < Error = Fut :: Error > , { fn is_terminated (& self) -> bool { matches ! (self , Self :: Empty) } }
    };
}

impl_118!();