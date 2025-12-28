macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! impl_885 {
    () => {
        deps!();
        impl < Fut : Future > FusedStream for FuturesUnordered < Fut > { fn is_terminated (& self) -> bool { self . is_terminated . load (Relaxed) } }
    };
}

impl_885!()