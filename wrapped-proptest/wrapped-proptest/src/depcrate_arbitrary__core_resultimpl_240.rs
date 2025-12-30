// Generated macro for impl_240 (impl)
macro_rules! Depcrate_arbitrary__core_resultimpl_240 {
() => {
// Module: crate::arbitrary::_core::result
// Provides: {"impl_240"}
// Dependencies: {}
impl < A : fmt :: Debug , E : Arbitrary > functor :: ArbitraryF1 < A > for Result < A , E > where E :: Strategy : 'static , { type Parameters = product_type ! [Probability , E :: Parameters] ; fn lift1_with < AS > (base : AS , args : Self :: Parameters) -> BoxedStrategy < Self > where AS : Strategy < Value = A > + 'static , { let product_unpack ! [prob , e] = args ; let (p , a , e) = (prob , base , any_with :: < E > (e)) ; maybe_ok_weighted (p , a , e) . boxed () } }
};
}
