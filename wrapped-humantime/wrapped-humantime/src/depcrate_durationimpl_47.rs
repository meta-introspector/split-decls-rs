// Generated macro for impl_47 (impl)
macro_rules! Depcrate_durationimpl_47 {
() => {
// Module: crate::duration
// Provides: {"impl_47"}
// Dependencies: {}
impl fmt :: Display for FormattedDuration { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let secs = self . 0 . as_secs () ; let nanos = self . 0 . subsec_nanos () ; if secs == 0 && nanos == 0 { f . write_str ("0s") ? ; return Ok (()) ; } let years = secs / 31_557_600 ; let ydays = secs % 31_557_600 ; let months = ydays / 2_630_016 ; let mdays = ydays % 2_630_016 ; let days = mdays / 86400 ; let day_secs = mdays % 86400 ; let hours = day_secs / 3600 ; let minutes = day_secs % 3600 / 60 ; let seconds = day_secs % 60 ; let millis = nanos / 1_000_000 ; let micros = nanos / 1000 % 1000 ; let nanosec = nanos % 1000 ; let started = & mut false ; item_plural (f , started , "year" , years) ? ; item_plural (f , started , "month" , months) ? ; item_plural (f , started , "day" , days) ? ; item (f , started , "h" , hours as u32) ? ; item (f , started , "m" , minutes as u32) ? ; item (f , started , "s" , seconds as u32) ? ; item (f , started , "ms" , millis) ? ; # [cfg (feature = "mu")] item (f , started , "µs" , micros) ? ; # [cfg (not (feature = "mu"))] item (f , started , "us" , micros) ? ; item (f , started , "ns" , nanosec) ? ; Ok (()) } }
};
}
