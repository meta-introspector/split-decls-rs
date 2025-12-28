macro_rules! PollFn {
    () => {
        pub (crate) struct PollFn < F > { f : F , }
    };
}

PollFn!()