// Generated macro for is_fast (function)
macro_rules! Depcrate_arch_all_rabinkarpis_fast {
() => {
// Module: crate::arch::all::rabinkarp
// Provides: {"is_fast"}
// Dependencies: {}
# [doc = " Whether RK is believed to be very fast for the given needle/haystack."] # [inline] pub (crate) fn is_fast (haystack : & [u8] , _needle : & [u8]) -> bool { haystack . len () < 16 }
};
}
