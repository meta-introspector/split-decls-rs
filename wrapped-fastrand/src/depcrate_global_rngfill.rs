// Generated macro for fill (function)
macro_rules! Depcrate_global_rngfill {
() => {
// Module: crate::global_rng
// Provides: {"fill"}
// Dependencies: {}
# [doc = " Fill a byte slice with random data."] # [inline] pub fn fill (slice : & mut [u8]) { with_rng (| r | r . fill (slice)) }
};
}
