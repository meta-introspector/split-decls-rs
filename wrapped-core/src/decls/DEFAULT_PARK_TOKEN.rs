macro_rules! deps {
    () => {
        ParkToken!();
    };
}

macro_rules! DEFAULT_PARK_TOKEN {
    () => {
        deps!();
        # [doc = " A default park token to use."] pub const DEFAULT_PARK_TOKEN : ParkToken = ParkToken (0) ;
    };
}

DEFAULT_PARK_TOKEN!();