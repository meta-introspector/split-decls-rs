macro_rules! deps {
    () => {
        TimeSpec!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl TimeSpec { # [doc = " Leave the timestamp unchanged."] # [cfg (not (target_os = "redox"))] pub const UTIME_OMIT : TimeSpec = TimeSpec :: new (0 , libc :: UTIME_OMIT as timespec_tv_nsec_t) ; # [doc = " Update the timestamp to `Now`"] # [cfg (not (target_os = "redox"))] pub const UTIME_NOW : TimeSpec = TimeSpec :: new (0 , libc :: UTIME_NOW as timespec_tv_nsec_t) ; # [doc = " Construct a new `TimeSpec` from its components"] # [cfg_attr (any (target_env = "musl" , target_env = "ohos") , allow (deprecated))] pub const fn new (seconds : time_t , nanoseconds : timespec_tv_nsec_t) -> Self { let mut ts = zero_init_timespec () ; ts . tv_sec = seconds ; ts . tv_nsec = nanoseconds ; Self (ts) } fn nanos_mod_sec (& self) -> timespec_tv_nsec_t { if self . tv_sec () < 0 && self . tv_nsec () > 0 { self . tv_nsec () - NANOS_PER_SEC as timespec_tv_nsec_t } else { self . tv_nsec () } } # [cfg_attr (any (target_env = "musl" , target_env = "ohos") , allow (deprecated))] pub const fn tv_sec (& self) -> time_t { self . 0 . tv_sec } pub const fn tv_nsec (& self) -> timespec_tv_nsec_t { self . 0 . tv_nsec } # [cfg_attr (any (target_env = "musl" , target_env = "ohos") , allow (deprecated))] pub const fn from_duration (duration : Duration) -> Self { let mut ts = zero_init_timespec () ; ts . tv_sec = duration . as_secs () as time_t ; ts . tv_nsec = duration . subsec_nanos () as timespec_tv_nsec_t ; TimeSpec (ts) } pub const fn from_timespec (timespec : timespec) -> Self { Self (timespec) } }
    };
}

impl_166!()