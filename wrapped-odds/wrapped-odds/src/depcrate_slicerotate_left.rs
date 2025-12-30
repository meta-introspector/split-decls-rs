// Generated macro for rotate_left (function)
macro_rules! Depcrate_slicerotate_left {
() => {
// Module: crate::slice
// Provides: {"rotate_left"}
// Dependencies: {}
# [doc = " Rotate `steps` towards lower indices."] # [doc = ""] # [doc = " The steps to rotate is computed modulo the length of `data`,"] # [doc = " so any step value is acceptable. This function does not panic."] # [doc = ""] # [doc = " ```"] # [doc = " use odds::slice::rotate_left;"] # [doc = ""] # [doc = " let mut data = [1, 2, 3, 4];"] # [doc = " rotate_left(&mut data, 1);"] # [doc = " assert_eq!(&data, &[2, 3, 4, 1]);"] # [doc = " rotate_left(&mut data, 2);"] # [doc = " assert_eq!(&data, &[4, 1, 2, 3]);"] # [doc = " ```"] pub fn rotate_left < T > (data : & mut [T] , steps : usize) { if data . len () == 0 { return ; } let steps = steps % data . len () ; data [.. steps] . reverse () ; data [steps ..] . reverse () ; data . reverse () ; }
};
}
