// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl Buffer { const fn new () -> Buffer { Buffer { inner : UnsafeCell :: new ([0 ; BUF_SIZE]) , } } const fn get (& self) -> * mut u8 { self . inner . get () as _ } }
};
}
