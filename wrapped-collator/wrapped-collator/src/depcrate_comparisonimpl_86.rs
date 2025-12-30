// Generated macro for impl_86 (impl)
macro_rules! Depcrate_comparisonimpl_86 {
() => {
// Module: crate::comparison
// Provides: {"impl_86"}
// Dependencies: {}
impl AnyQuaternaryAccumulator { # [inline (always)] pub fn new () -> Self { AnyQuaternaryAccumulator (0) } # [inline (always)] pub fn accumulate (& mut self , non_primary : NonPrimary) { self . 0 |= non_primary . bits () } # [inline (always)] pub fn has_quaternary (& self) -> bool { self . 0 & u32 :: from (QUATERNARY_MASK) != 0 } }
};
}
