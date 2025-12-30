// Generated macro for impl_996 (impl)
macro_rules! Depcrate_tz_offsetimpl_996 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_996"}
// Dependencies: {}
# [doc = " Computes the span of time between two offsets."] # [doc = ""] # [doc = " This will return a negative span when the offset being subtracted is"] # [doc = " greater (i.e., more east with respect to the prime meridian)."] impl Sub for Offset { type Output = Span ; # [inline] fn sub (self , rhs : Offset) -> Span { self . since (rhs) } }
};
}
