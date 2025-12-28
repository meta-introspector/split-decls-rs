macro_rules! deps {
    () => {
        Profiler!();
    };
}

macro_rules! EventIdBuilder {
    () => {
        deps!();
        pub struct EventIdBuilder < 'p > { profiler : & 'p Profiler , }
    };
}

EventIdBuilder!()