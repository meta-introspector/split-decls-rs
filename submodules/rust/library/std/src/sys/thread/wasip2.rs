mkuse!{use crate :: time :: { Duration , Instant } ;}

macro_rules! sleep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sleep in module {}", module_path!());
    };
}

mkfn!{
    sleep_introspect!();
    pub fn sleep (dur : Duration) { let mut remaining = dur . as_nanos () ; while remaining > 0 { let amt = u64 :: try_from (remaining) . unwrap_or (u64 :: MAX) ; wasip2 :: clocks :: monotonic_clock :: subscribe_duration (amt) . block () ; remaining -= u128 :: from (amt) ; } }
}

macro_rules! sleep_until_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sleep_until in module {}", module_path!());
    };
}

mkfn!{
    sleep_until_introspect!();
    pub fn sleep_until (deadline : Instant) { match u64 :: try_from (deadline . into_inner () . as_duration () . as_nanos ()) { Ok (deadline) => { wasip2 :: clocks :: monotonic_clock :: subscribe_instant (deadline) . block () ; } Err (_) => { let now = Instant :: now () ; if let Some (delay) = deadline . checked_duration_since (now) { sleep (delay) ; } } } }
}