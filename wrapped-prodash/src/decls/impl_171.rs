macro_rules! deps {
    () => {
        Count!();
        StepShared!();
        ThroughputOnDrop!();
        NestedProgress!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < T : NestedProgress > Count for ThroughputOnDrop < T > { fn set (& self , step : usize) { self . 0 . set (step) } fn step (& self) -> usize { self . 0 . step () } fn inc_by (& self , step : usize) { self . 0 . inc_by (step) } fn counter (& self) -> StepShared { self . 0 . counter () } }
    };
}

impl_171!()