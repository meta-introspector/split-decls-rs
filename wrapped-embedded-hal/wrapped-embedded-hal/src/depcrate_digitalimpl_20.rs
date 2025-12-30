// Generated macro for impl_20 (impl)
macro_rules! Depcrate_digitalimpl_20 {
() => {
// Module: crate::digital
// Provides: {"impl_20"}
// Dependencies: {}
impl Not for PinState { type Output = PinState ; # [inline] fn not (self) -> Self :: Output { match self { PinState :: High => PinState :: Low , PinState :: Low => PinState :: High , } } }
};
}
