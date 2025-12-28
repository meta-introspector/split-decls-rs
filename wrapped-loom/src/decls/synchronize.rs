macro_rules! deps {
    () => {
        Execution!();
    };
}

macro_rules! synchronize {
    () => {
        deps!();
        fn synchronize < F , R > (f : F) -> R where F : FnOnce (& mut Execution) -> R , { execution (| execution | { execution . threads . active_causality_inc () ; trace ! ("synchronize") ; f (execution) }) }
    };
}

synchronize!();