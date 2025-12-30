// Generated macro for AbbreviationsCache (struct)
macro_rules! Depcrate_read_abbrevAbbreviationsCache {
() => {
// Module: crate::read::abbrev
// Provides: {"AbbreviationsCache"}
// Dependencies: {}
# [doc = " A cache of previously parsed `Abbreviations`."] # [derive (Debug , Default)] pub struct AbbreviationsCache { abbreviations : btree_map :: BTreeMap < u64 , Result < Arc < Abbreviations > > > , }
};
}
