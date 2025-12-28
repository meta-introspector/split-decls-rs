macro_rules! deps {
    () => {
        MultiThreaded!();
    };
}

macro_rules! DefaultThreadMode {
    () => {
        deps!();
        # [cfg (feature = "multi-threaded-cf")] type DefaultThreadMode = crate :: MultiThreaded ;
    };
}

DefaultThreadMode!()