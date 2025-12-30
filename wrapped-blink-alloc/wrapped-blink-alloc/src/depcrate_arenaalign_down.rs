// Generated macro for align_down (function)
macro_rules! Depcrate_arenaalign_down {
() => {
// Module: crate::arena
// Provides: {"align_down"}
// Dependencies: {}
# [inline (always)] fn align_down (value : usize , align : usize) -> usize { debug_assert ! (align . is_power_of_two ()) ; let mask = align - 1 ; value & ! mask }
};
}
