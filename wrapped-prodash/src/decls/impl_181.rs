macro_rules! deps {
    () => {
        Log!();
        StepShared!();
        Step!();
        Count!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl Count for Log { fn set (& self , step : Step) { self . step . store (step , Ordering :: SeqCst) ; self . maybe_log () } fn step (& self) -> usize { self . step . load (Ordering :: Relaxed) } fn inc_by (& self , step : Step) { self . step . fetch_add (step , Ordering :: Relaxed) ; self . maybe_log () } fn counter (& self) -> StepShared { self . step . clone () } }
    };
}

impl_181!()