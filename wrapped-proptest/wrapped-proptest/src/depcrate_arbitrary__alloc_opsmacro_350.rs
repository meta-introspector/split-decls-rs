// Generated macro for macro_350 (macro)
macro_rules! Depcrate_arbitrary__alloc_opsmacro_350 {
() => {
// Module: crate::arbitrary::_alloc::ops
// Provides: {"macro_350"}
// Dependencies: {}
arbitrary ! ([A : PartialOrd + Arbitrary] RangeInclusive < A >, SMapped < (A , A) , Self >, product_type ! [A :: Parameters , A :: Parameters] ; args => static_map (any_with ::< (A , A) > (args) , | (a , b) | if b < a { b ..= a } else { a ..= b })) ;
};
}
