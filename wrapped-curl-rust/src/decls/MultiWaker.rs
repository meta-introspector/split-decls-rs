macro_rules! deps {
    () => {
        RawMulti!();
    };
}

macro_rules! MultiWaker {
    () => {
        deps!();
        # [doc = " A handle that can be used to wake up a thread that's blocked in [Multi::poll]."] # [doc = " The handle can be passed to and used from any thread."] # [cfg (feature = "poll_7_68_0")] # [derive (Debug , Clone)] pub struct MultiWaker { raw : std :: sync :: Weak < RawMulti > , }
    };
}

MultiWaker!()