// Generated macro for macro_353 (macro)
macro_rules! Depcrate_arbitrary__alloc_opsmacro_353 {
() => {
// Module: crate::arbitrary::_alloc::ops
// Provides: {"macro_353"}
// Dependencies: {}
lift1 ! ([PartialOrd] Range < A >; base => { let base = Arc :: new (base) ; (base . clone () , base) . prop_map (| (a , b) | if b < a { b .. a } else { a .. b }) }) ;
};
}
