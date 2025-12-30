// Generated macro for impl_364 (impl)
macro_rules! Depcrate_fmt_offsetimpl_364 {
() => {
// Module: crate::fmt::offset
// Provides: {"impl_364"}
// Dependencies: {}
impl Numeric { # [doc = " Convert a parsed numeric offset into a Jiff offset."] # [doc = ""] # [doc = " This does rounding based on the fractional nanosecond part. As a"] # [doc = " result, if the parsed value would be rounded to a value not in bounds"] # [doc = " for a Jiff offset, this returns an error."] fn to_offset (& self) -> Result < Offset , Error > { let mut seconds = t :: SpanZoneOffset :: rfrom (C (3_600) * self . hours) ; if let Some (part_minutes) = self . minutes { seconds += C (60) * part_minutes ; } if let Some (part_seconds) = self . seconds { seconds += part_seconds ; } if let Some (part_nanoseconds) = self . nanoseconds { if part_nanoseconds >= C (500_000_000) { seconds = seconds . try_checked_add ("offset-seconds" , C (1)) . with_context (| | { err ! ("due to precision loss, UTC offset '{}' is \
                             rounded to a value that is out of bounds" , self ,) }) ? ; } } Ok (Offset :: from_seconds_ranged (seconds * self . sign)) } }
};
}
