macro_rules! deps {
    () => {
        Duration!();
    };
}

macro_rules! ONCE_A_SECOND {
    () => {
        deps!();
        const ONCE_A_SECOND : Duration = Duration :: from_secs (1) ;
    };
}

ONCE_A_SECOND!();