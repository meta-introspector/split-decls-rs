// Generated macro for macro_239 (macro)
macro_rules! Depcrate_arbitrary__core_resultmacro_239 {
() => {
// Module: crate::arbitrary::_core::result
// Provides: {"macro_239"}
// Dependencies: {}
arbitrary ! ([A : Arbitrary , B : Arbitrary] Result < A , B >, MaybeOk < A :: Strategy , B :: Strategy >, product_type ! [Probability , A :: Parameters , B :: Parameters] ; args => { let product_unpack ! [prob , a , b] = args ; let (p , a , b) = (prob , any_with ::< A > (a) , any_with ::< B > (b)) ; maybe_ok_weighted (p , a , b) }) ;
};
}
