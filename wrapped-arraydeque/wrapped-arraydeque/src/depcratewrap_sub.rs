// Generated macro for wrap_sub (function)
macro_rules! Depcratewrap_sub {
() => {
// Module: crate
// Provides: {"wrap_sub"}
// Dependencies: {}
# [inline] fn wrap_sub (index : usize , subtrahend : usize , capacity : usize) -> usize { debug_assert ! (subtrahend <= capacity) ; (index + capacity - subtrahend) % capacity }
};
}
