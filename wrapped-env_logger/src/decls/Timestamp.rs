macro_rules! deps {
    () => {
        Formatter!();
        TimestampPrecision!();
    };
}

macro_rules! Timestamp {
    () => {
        deps!();
        # [doc = " An [RFC3339] formatted timestamp."] # [doc = ""] # [doc = " The timestamp implements [`Display`] and can be written to a [`Formatter`]."] # [doc = ""] # [doc = " [RFC3339]: https://www.ietf.org/rfc/rfc3339.txt"] # [doc = " [`Display`]: std::fmt::Display"] pub struct Timestamp { time : SystemTime , precision : TimestampPrecision , }
    };
}

Timestamp!()