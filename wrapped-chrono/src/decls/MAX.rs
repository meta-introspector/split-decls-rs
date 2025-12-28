macro_rules! deps {
    () => {
        TimeDelta!();
    };
}

macro_rules! MAX {
    () => {
        deps!();
        # [doc = " The maximum possible `TimeDelta`: `i64::MAX` milliseconds."] pub (crate) const MAX : TimeDelta = TimeDelta { secs : i64 :: MAX / MILLIS_PER_SEC , nanos : (i64 :: MAX % MILLIS_PER_SEC) as i32 * NANOS_PER_MILLI , } ;
    };
}

MAX!()