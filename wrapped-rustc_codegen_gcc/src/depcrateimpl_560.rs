// Generated macro for impl_560 (impl)
macro_rules! Depcrateimpl_560 {
() => {
// Module: crate
// Provides: {"impl_560"}
// Dependencies: {}
impl < F : Fn () -> String > Drop for PrintOnPanic < F > { fn drop (& mut self) { if :: std :: thread :: panicking () { println ! ("{}" , (self . 0) ()) ; } } }
};
}
