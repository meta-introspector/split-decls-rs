macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Time {
    () => {
        deps!();
        # [doc = " A key which represents a date."] pub type Time = Any < validate :: Time > ;
    };
}

Time!();