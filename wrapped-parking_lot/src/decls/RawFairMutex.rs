macro_rules! deps {
    () => {
        RawMutex!();
    };
}

macro_rules! RawFairMutex {
    () => {
        deps!();
        # [doc = " Raw fair mutex type backed by the parking lot."] pub struct RawFairMutex (RawMutex) ;
    };
}

RawFairMutex!()