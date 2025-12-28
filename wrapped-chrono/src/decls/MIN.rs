macro_rules! deps {
    () => {
        TimeDelta!();
    };
}

macro_rules! MIN {
    () => {
        deps!();
        # [doc = " The minimum possible `TimeDelta`: `-i64::MAX` milliseconds."] pub (crate) const MIN : TimeDelta = TimeDelta { secs : - i64 :: MAX / MILLIS_PER_SEC - 1 , nanos : NANOS_PER_SEC + (- i64 :: MAX % MILLIS_PER_SEC) as i32 * NANOS_PER_MILLI , } ;
    };
}

MIN!()