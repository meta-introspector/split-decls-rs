macro_rules! deps {
    () => {
        TimeValLike!();
        TimeSpec!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl TimeValLike for TimeSpec { # [inline] # [cfg_attr (any (target_env = "musl" , target_env = "ohos") , allow (deprecated))] fn seconds (seconds : i64) -> TimeSpec { assert ! ((TS_MIN_SECONDS ..= TS_MAX_SECONDS) . contains (& seconds) , "TimeSpec out of bounds; seconds={seconds}" ,) ; let mut ts = zero_init_timespec () ; ts . tv_sec = seconds as time_t ; TimeSpec (ts) } # [inline] fn milliseconds (milliseconds : i64) -> TimeSpec { let nanoseconds = milliseconds . checked_mul (1_000_000) . expect ("TimeSpec::milliseconds out of bounds") ; TimeSpec :: nanoseconds (nanoseconds) } # [doc = " Makes a new `TimeSpec` with given number of microseconds."] # [inline] fn microseconds (microseconds : i64) -> TimeSpec { let nanoseconds = microseconds . checked_mul (1_000) . expect ("TimeSpec::milliseconds out of bounds") ; TimeSpec :: nanoseconds (nanoseconds) } # [doc = " Makes a new `TimeSpec` with given number of nanoseconds."] # [inline] # [cfg_attr (any (target_env = "musl" , target_env = "ohos") , allow (deprecated))] fn nanoseconds (nanoseconds : i64) -> TimeSpec { let (secs , nanos) = div_mod_floor_64 (nanoseconds , NANOS_PER_SEC) ; assert ! ((TS_MIN_SECONDS ..= TS_MAX_SECONDS) . contains (& secs) , "TimeSpec out of bounds") ; let mut ts = zero_init_timespec () ; ts . tv_sec = secs as time_t ; ts . tv_nsec = nanos as timespec_tv_nsec_t ; TimeSpec (ts) } # [allow (clippy :: unnecessary_cast)] fn num_seconds (& self) -> i64 { if self . tv_sec () < 0 && self . tv_nsec () > 0 { (self . tv_sec () + 1) as i64 } else { self . tv_sec () as i64 } } fn num_milliseconds (& self) -> i64 { self . num_nanoseconds () / 1_000_000 } fn num_microseconds (& self) -> i64 { self . num_nanoseconds () / 1_000 } # [allow (clippy :: unnecessary_cast)] fn num_nanoseconds (& self) -> i64 { let secs = self . num_seconds () * 1_000_000_000 ; let nsec = self . nanos_mod_sec () ; secs + nsec as i64 } }
    };
}

impl_165!();