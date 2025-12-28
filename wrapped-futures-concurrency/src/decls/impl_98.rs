macro_rules! deps {
    () => {
        InlineWakerVec!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl Wake for InlineWakerVec { fn wake (self : Arc < Self >) { let mut readiness = self . readiness . lock () . unwrap () ; if ! readiness . set_ready (self . id) { readiness . parent_waker () . expect ("`parent_waker` not available from `Readiness`. Did you forget to call `Readiness::set_waker`?") . wake_by_ref () } } }
    };
}

impl_98!();