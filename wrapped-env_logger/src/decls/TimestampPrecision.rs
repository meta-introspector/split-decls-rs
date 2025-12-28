macro_rules! TimestampPrecision {
    () => {
        # [doc = " Formatting precision of timestamps."] # [doc = ""] # [doc = " Seconds give precision of full seconds, milliseconds give thousands of a"] # [doc = " second (3 decimal digits), microseconds are millionth of a second (6 decimal"] # [doc = " digits) and nanoseconds are billionth of a second (9 decimal digits)."] # [allow (clippy :: exhaustive_enums)] # [derive (Copy , Clone , Debug)] pub enum TimestampPrecision { # [doc = " Full second precision (0 decimal digits)"] Seconds , # [doc = " Millisecond precision (3 decimal digits)"] Millis , # [doc = " Microsecond precision (6 decimal digits)"] Micros , # [doc = " Nanosecond precision (9 decimal digits)"] Nanos , }
    };
}

TimestampPrecision!();