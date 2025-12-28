macro_rules! deps {
    () => {
        InlineWakerArray!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < const N : usize > Wake for InlineWakerArray < N > { fn wake (self : Arc < Self >) { let mut readiness = self . readiness . lock () . unwrap () ; if ! readiness . set_ready (self . id) { readiness . parent_waker () . expect ("`parent_waker` not available from `Readiness`. Did you forget to call `Readiness::set_waker`?") . wake_by_ref () } } }
    };
}

impl_74!()