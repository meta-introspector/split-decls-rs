macro_rules! deps {
    () => {
        ReadinessVec!();
    };
}

macro_rules! InlineWakerVec {
    () => {
        deps!();
        # [doc = " An efficient waker which delegates wake events."] # [derive (Debug , Clone)] pub (crate) struct InlineWakerVec { pub (crate) id : usize , pub (crate) readiness : Arc < Mutex < ReadinessVec > > , }
    };
}

InlineWakerVec!()