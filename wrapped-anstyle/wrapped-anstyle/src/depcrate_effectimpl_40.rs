// Generated macro for impl_40 (impl)
macro_rules! Depcrate_effectimpl_40 {
() => {
// Module: crate::effect
// Provides: {"impl_40"}
// Dependencies: {}
impl Iterator for EffectIter { type Item = Effects ; fn next (& mut self) -> Option < Self :: Item > { while self . index < METADATA . len () { let index = self . index ; self . index += 1 ; let effect = Effects (1 << index) ; if self . effects . contains (effect) { return Some (effect) ; } } None } }
};
}
