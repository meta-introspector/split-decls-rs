// Generated macro for impl_312 (impl)
macro_rules! Depcrate_outputimpl_312 {
() => {
// Module: crate::output
// Provides: {"impl_312"}
// Dependencies: {}
impl StringOutput { pub fn new () -> StringOutput { StringOutput { buf : Vec :: with_capacity (8 * 1024) , } } pub fn into_string (self) -> Result < String , FromUtf8Error > { String :: from_utf8 (self . buf) } }
};
}
