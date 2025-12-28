macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < Fut > FusedStream for TryFlatten < Fut , Fut :: Ok > where Fut : TryFuture , Fut :: Ok : TryStream < Error = Fut :: Error > , { fn is_terminated (& self) -> bool { matches ! (self , Self :: Empty) } }
    };
}

impl_120!();