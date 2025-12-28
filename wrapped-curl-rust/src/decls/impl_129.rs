macro_rules! deps {
    () => {
        MultiWaker!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        # [cfg (feature = "poll_7_68_0")] unsafe impl Sync for MultiWaker { }
    };
}

impl_129!();