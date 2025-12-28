macro_rules! deps {
    () => {
        Item!();
        Count!();
        StepShared!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl crate :: Count for Item { fn set (& self , step : usize) { Item :: set (self , step) } fn step (& self) -> usize { Item :: step (self) . unwrap_or (0) } fn inc_by (& self , step : usize) { self . inc_by (step) } fn counter (& self) -> StepShared { Arc :: clone (& self . value) } }
    };
}

impl_8!()