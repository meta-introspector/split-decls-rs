macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! Configuration {
    () => {
        deps!();
        # [doc = " Contains the rayon thread pool configuration. Use [`ThreadPoolBuilder`] instead."] # [deprecated (note = "Use `ThreadPoolBuilder`")] # [derive (Default)] pub struct Configuration { builder : ThreadPoolBuilder , }
    };
}

Configuration!();