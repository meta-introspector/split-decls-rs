// Generated macro for align_up (function)
macro_rules! Depcratealign_up {
() => {
// Module: crate
// Provides: {"align_up"}
// Dependencies: {}
# [inline (always)] fn align_up (val : usize , align : usize) -> usize { debug_assert ! (align . is_power_of_two ()) ; (val + align - 1) & ! (align - 1) }
};
}
