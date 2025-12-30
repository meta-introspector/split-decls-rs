// Generated macro for impl_66 (impl)
macro_rules! Depcrate_features_proptestimpl_66 {
() => {
// Module: crate::features::proptest
// Provides: {"impl_66"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "proptest")))] impl Arbitrary for CompactString { type Parameters = StringParam ; type Strategy = MapInto < StrategyFor < String > , Self > ; fn arbitrary_with (a : Self :: Parameters) -> Self :: Strategy { any_with :: < String > (a) . prop_map_into () } }
};
}
