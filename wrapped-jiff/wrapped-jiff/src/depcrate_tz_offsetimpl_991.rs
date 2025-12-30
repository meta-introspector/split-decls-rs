// Generated macro for impl_991 (impl)
macro_rules! Depcrate_tz_offsetimpl_991 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_991"}
// Dependencies: {}
impl core :: fmt :: Display for Offset { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let sign = if self . span < C (0) { "-" } else { "+" } ; let hours = self . part_hours_ranged () . abs () . get () ; let minutes = self . part_minutes_ranged () . abs () . get () ; let seconds = self . part_seconds_ranged () . abs () . get () ; if hours == 0 && minutes == 0 && seconds == 0 { write ! (f , "+00") } else if hours != 0 && minutes == 0 && seconds == 0 { write ! (f , "{sign}{hours:02}") } else if minutes != 0 && seconds == 0 { write ! (f , "{sign}{hours:02}:{minutes:02}") } else { write ! (f , "{sign}{hours:02}:{minutes:02}:{seconds:02}") } } }
};
}
