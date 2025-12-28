macro_rules! deps {
    () => {
        ReadinessVec!();
    };
}

macro_rules! WakerVec {
    () => {
        deps!();
        # [doc = " A collection of wakers which delegate to an in-line waker."] pub (crate) struct WakerVec { wakers : Vec < Waker > , readiness : Arc < Mutex < ReadinessVec > > , }
    };
}

WakerVec!()