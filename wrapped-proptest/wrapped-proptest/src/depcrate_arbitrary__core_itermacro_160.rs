// Generated macro for macro_160 (macro)
macro_rules! Depcrate_arbitrary__core_itermacro_160 {
() => {
// Module: crate::arbitrary::_core::iter
// Provides: {"macro_160"}
// Dependencies: {}
arbitrary ! ([T , A : Arbitrary + Iterator < Item = T >, B : Arbitrary + Iterator < Item = T >] Chain < A , B >, SMapped < (A , B) , Self >, product_type ! [A :: Parameters , B :: Parameters] ; args => static_map (any_with ::< (A , B) > (args) , | (a , b) | a . chain (b))) ;
};
}
