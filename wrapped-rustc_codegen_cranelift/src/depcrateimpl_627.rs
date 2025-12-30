// Generated macro for impl_627 (impl)
macro_rules! Depcrateimpl_627 {
() => {
// Module: crate
// Provides: {"impl_627"}
// Dependencies: {}
impl < F : Fn () -> String > Drop for PrintOnPanic < F > { fn drop (& mut self) { if :: std :: thread :: panicking () { println ! ("{}" , (self . 0) ()) ; } } }
};
}
