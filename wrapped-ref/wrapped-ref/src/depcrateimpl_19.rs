// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl < V > Ref < V > { # [doc = "\n    Wrap a value.\n    "] pub fn new (value : V) -> Self { Ref (value) } # [doc = "\n    Get a reference to the underlying value.\n    "] pub fn inner (& self) -> & V { & self . 0 } # [doc = "\n    Take ownership of the underlying value.\n    "] pub fn into_inner (self) -> V { self . 0 } }
};
}
