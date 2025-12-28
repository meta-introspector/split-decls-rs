macro_rules! deps {
    () => {
        LocalHandle!();
    };
}

macro_rules! macro_147 {
    () => {
        deps!();
        thread_local ! { # [doc = " The per-thread participant for the default garbage collector."] static HANDLE : LocalHandle = collector () . register () ; }
    };
}

macro_147!()