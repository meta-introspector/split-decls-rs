macro_rules! deps {
    () => {
        Count!();
        DoOrDiscard!();
        StepShared!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < T > Count for DoOrDiscard < T > where T : Count , { fn set (& self , step : usize) { self . 0 . set (step) } fn step (& self) -> usize { self . 0 . step () } fn inc_by (& self , step : usize) { self . 0 . inc_by (step) } fn counter (& self) -> StepShared { self . 0 . counter () } }
    };
}

impl_166!()