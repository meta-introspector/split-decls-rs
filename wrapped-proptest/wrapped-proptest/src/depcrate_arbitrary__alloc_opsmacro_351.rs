// Generated macro for macro_351 (macro)
macro_rules! Depcrate_arbitrary__alloc_opsmacro_351 {
() => {
// Module: crate::arbitrary::_alloc::ops
// Provides: {"macro_351"}
// Dependencies: {}
lift1 ! ([PartialOrd] RangeInclusive < A >; base => { let base = Arc :: new (base) ; (base . clone () , base) . prop_map (| (a , b) | if b < a { b ..= a } else { a ..= b }) }) ;
};
}
