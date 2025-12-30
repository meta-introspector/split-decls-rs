// Generated macro for RuleCollectionProvider (struct)
macro_rules! Depcrate_transliterate_compileRuleCollectionProvider {
() => {
// Module: crate::transliterate::compile
// Provides: {"RuleCollectionProvider"}
// Dependencies: {}
# [doc = " A provider that is usable by [`Transliterator::try_new_unstable`](crate::transliterate::Transliterator::try_new_unstable)."] # [derive (Debug)] pub struct RuleCollectionProvider < 'a , PP : ? Sized , NP : ? Sized , NC : ? Sized > { collection : & 'a RuleCollection , properties_provider : & 'a PP , normalizer_provider : & 'a NP , casemap_provider : & 'a NC , xid_start : CodePointSetData , xid_continue : CodePointSetData , pat_ws : CodePointSetData , }
};
}
