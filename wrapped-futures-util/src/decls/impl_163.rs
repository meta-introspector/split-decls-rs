macro_rules! deps {
    () => {
        MaybeDone!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < Fut : Future > FusedFuture for MaybeDone < Fut > { fn is_terminated (& self) -> bool { match self { Self :: Future (_) => false , Self :: Done (_) | Self :: Gone => true , } } }
    };
}

impl_163!();