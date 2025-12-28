macro_rules! deps {
    () => {
        ReadinessArray!();
    };
}

macro_rules! WakerArray {
    () => {
        deps!();
        # [doc = " A collection of wakers which delegate to an in-line waker."] pub (crate) struct WakerArray < const N : usize > { wakers : [Waker ; N] , readiness : Arc < Mutex < ReadinessArray < N > > > , }
    };
}

WakerArray!();