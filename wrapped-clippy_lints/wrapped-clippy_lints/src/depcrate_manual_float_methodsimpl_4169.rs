// Generated macro for impl_4169 (impl)
macro_rules! Depcrate_manual_float_methodsimpl_4169 {
() => {
// Module: crate::manual_float_methods
// Provides: {"impl_4169"}
// Dependencies: {}
impl Variant { pub fn lint (self) -> & 'static Lint { match self { Self :: ManualIsInfinite => MANUAL_IS_INFINITE , Self :: ManualIsFinite => MANUAL_IS_FINITE , } } pub fn msg (self) -> & 'static str { match self { Self :: ManualIsInfinite => "manually checking if a float is infinite" , Self :: ManualIsFinite => "manually checking if a float is finite" , } } }
};
}
