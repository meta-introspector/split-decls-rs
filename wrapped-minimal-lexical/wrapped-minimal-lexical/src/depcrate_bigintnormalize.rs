// Generated macro for normalize (function)
macro_rules! Depcrate_bigintnormalize {
() => {
// Module: crate::bigint
// Provides: {"normalize"}
// Dependencies: {}
# [doc = " Normalize the integer, so any leading zero values are removed."] # [inline] pub fn normalize (x : & mut VecType) { while let Some (& value) = x . get (x . len () . wrapping_sub (1)) { if value == 0 { unsafe { x . set_len (x . len () - 1) } ; } else { break ; } } }
};
}
