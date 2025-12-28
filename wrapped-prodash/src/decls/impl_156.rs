macro_rules! deps {
    () => {
        Discard!();
        StepShared!();
        Count!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl Count for Discard { fn set (& self , _step : usize) { } fn step (& self) -> usize { 0 } fn inc_by (& self , _step : usize) { } fn counter (& self) -> StepShared { Arc :: new (AtomicUsize :: default ()) } }
    };
}

impl_156!();