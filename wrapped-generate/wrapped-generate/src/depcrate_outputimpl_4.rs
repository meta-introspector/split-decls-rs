// Generated macro for impl_4 (impl)
macro_rules! Depcrate_outputimpl_4 {
() => {
// Module: crate::output
// Provides: {"impl_4"}
// Dependencies: {}
impl Output { pub fn new () -> Self { Output (String :: new ()) } pub fn write_fmt (& mut self , arguments : fmt :: Arguments) { fmt :: Write :: write_fmt (& mut self . 0 , arguments) . unwrap () ; } }
};
}
