// Generated macro for impl_proptest (module)
macro_rules! Depcrateimpl_proptest {
() => {
// Module: crate
// Provides: {"impl_proptest"}
// Dependencies: {}
# [cfg (feature = "proptest")] mod impl_proptest { use super :: { NotNan , OrderedFloat } ; use proptest :: arbitrary :: { Arbitrary , StrategyFor } ; use proptest :: num :: { f32 , f64 } ; use proptest :: strategy :: { FilterMap , Map , Strategy } ; use std :: convert :: TryFrom ; macro_rules ! impl_arbitrary { ($ ($ f : ident) ,+) => { $ (impl Arbitrary for NotNan <$ f > { type Strategy = FilterMap < StrategyFor <$ f >, fn (_ : $ f) -> Option < NotNan <$ f >>>; type Parameters = <$ f as Arbitrary >:: Parameters ; fn arbitrary_with (params : Self :: Parameters) -> Self :: Strategy { <$ f >:: arbitrary_with (params) . prop_filter_map ("filter nan values" , | f | NotNan :: try_from (f) . ok ()) } } impl Arbitrary for OrderedFloat <$ f > { type Strategy = Map < StrategyFor <$ f >, fn (_ : $ f) -> OrderedFloat <$ f >>; type Parameters = <$ f as Arbitrary >:: Parameters ; fn arbitrary_with (params : Self :: Parameters) -> Self :: Strategy { <$ f >:: arbitrary_with (params) . prop_map (| f | OrderedFloat :: from (f)) } }) * } } impl_arbitrary ! { f32 , f64 } }
};
}
