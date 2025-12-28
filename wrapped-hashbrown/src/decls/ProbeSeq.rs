macro_rules! ProbeSeq {
    () => {
        # [doc = " Probe sequence based on triangular numbers, which is guaranteed (since our"] # [doc = " table size is a power of two) to visit every group of elements exactly once."] # [doc = ""] # [doc = " A triangular probe has us jump by 1 more group every time. So first we"] # [doc = " jump by 1 group (meaning we just continue our linear scan), then 2 groups"] # [doc = " (skipping over 1 group), then 3 groups (skipping over 2 groups), and so on."] # [doc = ""] # [doc = " Proof that the probe will visit every group in the table:"] # [doc = " <https://fgiesen.wordpress.com/2015/02/22/triangular-numbers-mod-2n/>"] # [derive (Clone)] struct ProbeSeq { pos : usize , stride : usize , }
    };
}

ProbeSeq!();