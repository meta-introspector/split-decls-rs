// Generated macro for impl_32 (impl)
macro_rules! Depcrate_dependencyimpl_32 {
() => {
// Module: crate::dependency
// Provides: {"impl_32"}
// Dependencies: {}
impl fmt :: Display for DependencyKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let s = serde_json :: to_string (self) . unwrap () ; f . write_str (& s [1 .. s . len () - 1]) } }
};
}
