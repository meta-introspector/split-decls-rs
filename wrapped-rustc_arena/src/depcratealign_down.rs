// Generated macro for align_down (function)
macro_rules! Depcratealign_down {
() => {
// Module: crate
// Provides: {"align_down"}
// Dependencies: {}
# [inline (always)] fn align_down (val : usize , align : usize) -> usize { debug_assert ! (align . is_power_of_two ()) ; val & ! (align - 1) }
};
}
