macro_rules! deps {
    () => {
        Execution!();
        Scheduler!();
    };
}

macro_rules! execution {
    () => {
        deps!();
        pub (crate) fn execution < F , R > (f : F) -> R where F : FnOnce (& mut Execution) -> R , { Scheduler :: with_execution (f) }
    };
}

execution!();