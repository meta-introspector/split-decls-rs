macro_rules! TimeValLike {
    () => {
        pub trait TimeValLike : Sized { # [inline] fn zero () -> Self { Self :: seconds (0) } # [inline] fn hours (hours : i64) -> Self { let secs = hours . checked_mul (SECS_PER_HOUR) . expect ("TimeValLike::hours ouf of bounds") ; Self :: seconds (secs) } # [inline] fn minutes (minutes : i64) -> Self { let secs = minutes . checked_mul (SECS_PER_MINUTE) . expect ("TimeValLike::minutes out of bounds") ; Self :: seconds (secs) } fn seconds (seconds : i64) -> Self ; fn milliseconds (milliseconds : i64) -> Self ; fn microseconds (microseconds : i64) -> Self ; fn nanoseconds (nanoseconds : i64) -> Self ; # [inline] fn num_hours (& self) -> i64 { self . num_seconds () / 3600 } # [inline] fn num_minutes (& self) -> i64 { self . num_seconds () / 60 } fn num_seconds (& self) -> i64 ; fn num_milliseconds (& self) -> i64 ; fn num_microseconds (& self) -> i64 ; fn num_nanoseconds (& self) -> i64 ; }
    };
}

TimeValLike!();