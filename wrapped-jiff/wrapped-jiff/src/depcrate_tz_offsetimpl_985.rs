// Generated macro for impl_985 (impl)
macro_rules! Depcrate_tz_offsetimpl_985 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_985"}
// Dependencies: {}
impl Dst { # [doc = " Returns true when this value is equal to `Dst::Yes`."] pub fn is_dst (self) -> bool { matches ! (self , Dst :: Yes) } # [doc = " Returns true when this value is equal to `Dst::No`."] # [doc = ""] # [doc = " `std` in this context refers to \"standard time.\" That is, it is the"] # [doc = " offset from UTC used when DST is not in effect."] pub fn is_std (self) -> bool { matches ! (self , Dst :: No) } }
};
}
