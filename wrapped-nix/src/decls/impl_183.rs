macro_rules! deps {
    () => {
        TimeVal!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl TimeVal { # [doc = " Construct a new `TimeVal` from its components"] # [cfg_attr (any (target_env = "musl" , target_env = "ohos") , allow (deprecated))] pub const fn new (seconds : time_t , microseconds : suseconds_t) -> Self { Self (timeval { tv_sec : seconds , tv_usec : microseconds , }) } fn micros_mod_sec (& self) -> suseconds_t { if self . tv_sec () < 0 && self . tv_usec () > 0 { self . tv_usec () - MICROS_PER_SEC as suseconds_t } else { self . tv_usec () } } # [cfg_attr (any (target_env = "musl" , target_env = "ohos") , allow (deprecated))] pub const fn tv_sec (& self) -> time_t { self . 0 . tv_sec } pub const fn tv_usec (& self) -> suseconds_t { self . 0 . tv_usec } }
    };
}

impl_183!();