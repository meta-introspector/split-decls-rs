macro_rules! deps {
    () => {
        ReadinessArray!();
    };
}

macro_rules! InlineWakerArray {
    () => {
        deps!();
        # [doc = " An efficient waker which delegates wake events."] # [derive (Debug , Clone)] pub (crate) struct InlineWakerArray < const N : usize > { pub (crate) id : usize , pub (crate) readiness : Arc < Mutex < ReadinessArray < N > > > , }
    };
}

InlineWakerArray!()