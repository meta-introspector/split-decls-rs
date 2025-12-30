// Generated macro for panic_does_not_fit (function)
macro_rules! Depcratepanic_does_not_fit {
() => {
// Module: crate
// Provides: {"panic_does_not_fit"}
// Dependencies: {}
# [cold] fn panic_does_not_fit (size : usize , nbytes : usize) -> ! { panic ! ("size too large: the integer type can fit {} bytes, but nbytes is {}" , size , nbytes) ; }
};
}
