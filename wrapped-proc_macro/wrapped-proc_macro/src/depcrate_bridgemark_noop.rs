// Generated macro for mark_noop (macro)
macro_rules! Depcrate_bridgemark_noop {
() => {
// Module: crate::bridge
// Provides: {"mark_noop"}
// Dependencies: {}
macro_rules ! mark_noop { ($ ($ ty : ty) ,* $ (,) ?) => { $ (impl Mark for $ ty { type Unmarked = Self ; fn mark (unmarked : Self :: Unmarked) -> Self { unmarked } } impl Unmark for $ ty { type Unmarked = Self ; fn unmark (self) -> Self :: Unmarked { self } }) * } }
};
}
