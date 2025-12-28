macro_rules! ReadinessArray {
    () => {
        # [doc = " Tracks which wakers are \"ready\" and should be polled."] # [derive (Debug)] pub (crate) struct ReadinessArray < const N : usize > { count : usize , readiness_list : [bool ; N] , parent_waker : Option < Waker > , }
    };
}

ReadinessArray!();