// Generated macro for macro_157 (macro)
macro_rules! Depcrate_arbitrary__core_itermacro_157 {
() => {
// Module: crate::arbitrary::_core::iter
// Provides: {"macro_157"}
// Dependencies: {}
arbitrary ! ([A : Arbitrary + Iterator , B : Arbitrary + Iterator] Zip < A , B >, SMapped < (A , B) , Self >, product_type ! [A :: Parameters , B :: Parameters] ; args => static_map (any_with ::< (A , B) > (args) , | (a , b) | a . zip (b))) ;
};
}
