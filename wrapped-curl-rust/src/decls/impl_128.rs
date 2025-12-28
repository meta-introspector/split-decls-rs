macro_rules! deps {
    () => {
        MultiWaker!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        # [cfg (feature = "poll_7_68_0")] unsafe impl Send for MultiWaker { }
    };
}

impl_128!()