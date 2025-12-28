macro_rules! deps {
    () => {
        TrackerData!();
    };
}

macro_rules! CguReuseTracker {
    () => {
        deps!();
        pub struct CguReuseTracker { data : Option < TrackerData > , }
    };
}

CguReuseTracker!();