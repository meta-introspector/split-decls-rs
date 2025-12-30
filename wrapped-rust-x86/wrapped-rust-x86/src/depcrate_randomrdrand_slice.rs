// Generated macro for rdrand_slice (function)
macro_rules! Depcrate_randomrdrand_slice {
() => {
// Module: crate::random
// Provides: {"rdrand_slice"}
// Dependencies: {}
# [doc = " Fill a slice with random values."] # [doc = ""] # [doc = " Returns true if the iterator was successfully filled with"] # [doc = " random values, otherwise false."] # [doc = " # Safety"] # [doc = " Will crash if RDRAND instructions are not supported."] pub unsafe fn rdrand_slice < T : RdRand > (buffer : & mut [T]) -> bool { let mut worked = true ; for element in buffer { worked &= element . fill_random () ; } worked }
};
}
