// Generated macro for align_up (function)
macro_rules! Depcrate_arenaalign_up {
() => {
// Module: crate::arena
// Provides: {"align_up"}
// Dependencies: {}
# [inline (always)] fn align_up (value : usize , align : usize) -> Option < usize > { debug_assert ! (align . is_power_of_two ()) ; let mask = align - 1 ; Some (value . checked_add (mask) ? & ! mask) }
};
}
