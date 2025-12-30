// Generated macro for alloc_guard (function)
macro_rules! Depcrate_raw_vecalloc_guard {
() => {
// Module: crate::raw_vec
// Provides: {"alloc_guard"}
// Dependencies: {}
# [inline (always)] fn alloc_guard (alloc_size : usize) -> Result < () , TryReserveError > { if usize :: BITS < 64 && alloc_size > isize :: MAX as usize { Err (CapacityOverflow . into ()) } else { Ok (()) } }
};
}
