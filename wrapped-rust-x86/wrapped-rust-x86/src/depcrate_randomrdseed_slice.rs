// Generated macro for rdseed_slice (function)
macro_rules! Depcrate_randomrdseed_slice {
() => {
// Module: crate::random
// Provides: {"rdseed_slice"}
// Dependencies: {}
# [doc = " Fill a slice with random values."] # [doc = ""] # [doc = " Returns true if the iterator was successfully filled with"] # [doc = " random values, otherwise false."] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDSEED instructions are not supported."] pub unsafe fn rdseed_slice < T : RdSeed > (buffer : & mut [T]) -> bool { let mut worked = true ; for element in buffer { worked &= element . fill_random () ; } worked }
};
}
