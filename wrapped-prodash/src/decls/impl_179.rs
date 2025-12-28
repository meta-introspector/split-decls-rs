macro_rules! deps {
    () => {
        Duration!();
        Log!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl Log { # [doc = " Create a new instance from `name` while displaying progress information only up to `max_level`."] pub fn new (name : impl Into < String > , max_level : Option < usize >) -> Self { let trigger = Arc :: new (AtomicBool :: new (true)) ; std :: thread :: spawn ({ let duration = Duration :: from_secs_f32 (EMIT_LOG_EVERY_S) ; let trigger = Arc :: downgrade (& trigger) ; move | | { while let Some (t) = trigger . upgrade () { t . store (true , Ordering :: Relaxed) ; std :: thread :: sleep (duration) ; } } }) ; Log { name : name . into () , id : crate :: progress :: UNKNOWN , current_level : 0 , max_level : max_level . unwrap_or (usize :: MAX) , max : None , step : Default :: default () , unit : None , trigger , } } }
    };
}

impl_179!();