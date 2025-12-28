macro_rules! deps {
    () => {
        Timer!();
    };
}

macro_rules! Time {
    () => {
        deps!();
        # [doc = " A user-provided timer to time background tasks."] # [derive (Clone)] pub (crate) enum Time { Timer (Arc < dyn Timer + Send + Sync >) , Empty , }
    };
}

Time!();