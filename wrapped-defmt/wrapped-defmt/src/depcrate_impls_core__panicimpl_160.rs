// Generated macro for impl_160 (impl)
macro_rules! Depcrate_impls_core__panicimpl_160 {
() => {
// Module: crate::impls::core_::panic
// Provides: {"impl_160"}
// Dependencies: {}
impl Format for panic :: PanicInfo < '_ > { fn format (& self , f : Formatter) { if let Some (location) = self . location () { crate :: write ! (f , "panicked at {}" , location) ; } else { crate :: write ! (f , "panicked") ; } } }
};
}
