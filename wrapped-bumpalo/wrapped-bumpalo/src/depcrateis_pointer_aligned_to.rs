// Generated macro for is_pointer_aligned_to (function)
macro_rules! Depcrateis_pointer_aligned_to {
() => {
// Module: crate
// Provides: {"is_pointer_aligned_to"}
// Dependencies: {}
# [inline] fn is_pointer_aligned_to < T > (pointer : * mut T , align : usize) -> bool { debug_assert ! (align . is_power_of_two ()) ; let pointer = pointer as usize ; let pointer_aligned = round_down_to (pointer , align) ; pointer == pointer_aligned }
};
}
