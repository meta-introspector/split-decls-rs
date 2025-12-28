macro_rules! deps {
    () => {
        PollFn!();
    };
}

macro_rules! poll_fn {
    () => {
        deps!();
        pub (crate) fn poll_fn < T , F > (f : F) -> PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < T > , { PollFn { f } }
    };
}

poll_fn!();