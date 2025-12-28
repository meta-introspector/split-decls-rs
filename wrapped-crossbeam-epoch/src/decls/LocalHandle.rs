macro_rules! deps {
    () => {
        Local!();
    };
}

macro_rules! LocalHandle {
    () => {
        deps!();
        # [doc = " A handle to a garbage collector."] pub struct LocalHandle { pub (crate) local : * const Local , }
    };
}

LocalHandle!()