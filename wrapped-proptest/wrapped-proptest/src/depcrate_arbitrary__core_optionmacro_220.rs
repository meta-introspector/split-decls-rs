// Generated macro for macro_220 (macro)
macro_rules! Depcrate_arbitrary__core_optionmacro_220 {
() => {
// Module: crate::arbitrary::_core::option
// Provides: {"macro_220"}
// Dependencies: {}
arbitrary ! ([A : Arbitrary] opt :: Option < A >, OptionStrategy < A :: Strategy >, product_type ! [Probability , A :: Parameters] ; args => { let product_unpack ! [prob , a] = args ; weighted (prob , any_with ::< A > (a)) }) ;
};
}
