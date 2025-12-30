// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl StringLeakExt for String { fn leak < 'a > (self) -> & 'a mut str { Box :: leak (self . into_boxed_str ()) } }
};
}
