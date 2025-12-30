// Generated macro for impl_161 (impl)
macro_rules! Depcrate_impls_core__panicimpl_161 {
() => {
// Module: crate::impls::core_::panic
// Provides: {"impl_161"}
// Dependencies: {}
impl Format for panic :: Location < '_ > { fn format (& self , f : Formatter) { crate :: write ! (f , "{=str}:{=u32}:{=u32}" , self . file () , self . line () , self . column ()) ; } }
};
}
