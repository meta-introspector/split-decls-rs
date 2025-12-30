// Generated macro for macro_352 (macro)
macro_rules! Depcrate_arbitrary__alloc_opsmacro_352 {
() => {
// Module: crate::arbitrary::_alloc::ops
// Provides: {"macro_352"}
// Dependencies: {}
arbitrary ! ([A : PartialOrd + Arbitrary] Range < A >, SMapped < (A , A) , Self >, product_type ! [A :: Parameters , A :: Parameters] ; args => static_map (any_with ::< (A , A) > (args) , | (a , b) | if b < a { b .. a } else { a .. b })) ;
};
}
