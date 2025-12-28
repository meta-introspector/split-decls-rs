macro_rules! deps {
    () => {
        TryMaybeDone!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < Fut : TryFuture > FusedFuture for TryMaybeDone < Fut > { fn is_terminated (& self) -> bool { match self { Self :: Future (_) => false , Self :: Done (_) | Self :: Gone => true , } } }
    };
}

impl_170!()