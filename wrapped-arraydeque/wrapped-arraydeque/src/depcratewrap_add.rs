// Generated macro for wrap_add (function)
macro_rules! Depcratewrap_add {
() => {
// Module: crate
// Provides: {"wrap_add"}
// Dependencies: {}
# [inline] fn wrap_add (index : usize , addend : usize , capacity : usize) -> usize { debug_assert ! (addend <= capacity) ; (index + addend) % capacity }
};
}
