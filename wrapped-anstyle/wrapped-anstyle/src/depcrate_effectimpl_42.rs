// Generated macro for impl_42 (impl)
macro_rules! Depcrate_effectimpl_42 {
() => {
// Module: crate::effect
// Provides: {"impl_42"}
// Dependencies: {}
impl Iterator for EffectIndexIter { type Item = usize ; fn next (& mut self) -> Option < Self :: Item > { while self . index < METADATA . len () { let index = self . index ; self . index += 1 ; let effect = Effects (1 << index) ; if self . effects . contains (effect) { return Some (index) ; } } None } }
};
}
