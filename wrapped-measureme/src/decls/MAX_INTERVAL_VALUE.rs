macro_rules! MAX_INTERVAL_VALUE {
    () => {
        # [doc = " The max value we can represent with the 48 bits available."] # [doc = " The highest two values are reserved for the `INSTANT_MARKER` and `INTEGER_MARKER`."] pub const MAX_INTERVAL_VALUE : u64 = INTEGER_MARKER - 1 ;
    };
}

MAX_INTERVAL_VALUE!()