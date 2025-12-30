// Generated macro for non_zero_impl (macro)
macro_rules! Depcrate_arbitrary__core_non_zeronon_zero_impl {
() => {
// Module: crate::arbitrary::_core::non_zero
// Provides: {"non_zero_impl"}
// Dependencies: {}
macro_rules ! non_zero_impl { ($ nz : ty , $ prim : ty) => { impl Arbitrary for $ nz { type Parameters = () ; type Strategy = FilterMap < StrategyFor <$ prim >, fn ($ prim) -> Option < Self >>; fn arbitrary_with (() : Self :: Parameters) -> Self :: Strategy { any ::<$ prim > () . prop_filter_map ("must be non zero" , | i | { Self :: try_from (i) . ok () }) } } } ; }
};
}
