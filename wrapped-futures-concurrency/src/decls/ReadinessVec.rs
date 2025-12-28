macro_rules! ReadinessVec {
    () => {
        # [doc = " Tracks which wakers are \"ready\" and should be polled."] # [derive (Debug)] pub (crate) struct ReadinessVec { ready_count : usize , max_count : usize , readiness_list : FixedBitSet , parent_waker : Option < Waker > , }
    };
}

ReadinessVec!()