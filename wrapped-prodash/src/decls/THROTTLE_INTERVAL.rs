macro_rules! deps {
    () => {
        Duration!();
    };
}

macro_rules! THROTTLE_INTERVAL {
    () => {
        deps!();
        const THROTTLE_INTERVAL : Duration = Duration :: from_secs (1) ;
    };
}

THROTTLE_INTERVAL!();