macro_rules! deps {
    () => {
        FuturesOrdered!();
    };
}

macro_rules! impl_829 {
    () => {
        deps!();
        impl < Fut : Future > FusedStream for FuturesOrdered < Fut > { fn is_terminated (& self) -> bool { self . in_progress_queue . is_terminated () && self . queued_outputs . is_empty () } }
    };
}

impl_829!()