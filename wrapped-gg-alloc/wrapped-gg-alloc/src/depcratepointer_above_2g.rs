// Generated macro for pointer_above_2g (function)
macro_rules! Depcratepointer_above_2g {
() => {
// Module: crate
// Provides: {"pointer_above_2g"}
// Dependencies: {}
fn pointer_above_2g (ptr : * mut u8) -> bool { (ptr as u32) > (i32 :: MAX as u32) }
};
}
