// Generated macro for is_aligned_to (function)
macro_rules! Depcrate_arenais_aligned_to {
() => {
// Module: crate::arena
// Provides: {"is_aligned_to"}
// Dependencies: {}
# [inline (always)] fn is_aligned_to (value : usize , align : usize) -> bool { debug_assert ! (align . is_power_of_two ()) ; let mask = align - 1 ; value & mask == 0 }
};
}
