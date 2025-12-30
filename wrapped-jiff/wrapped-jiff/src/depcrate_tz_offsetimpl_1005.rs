// Generated macro for impl_1005 (impl)
macro_rules! Depcrate_tz_offsetimpl_1005 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_1005"}
// Dependencies: {}
# [doc = " Negate this offset."] # [doc = ""] # [doc = " A positive offset becomes negative and vice versa. This is a no-op for the"] # [doc = " zero offset."] # [doc = ""] # [doc = " This never panics."] impl Neg for Offset { type Output = Offset ; # [inline] fn neg (self) -> Offset { self . negate () } }
};
}
