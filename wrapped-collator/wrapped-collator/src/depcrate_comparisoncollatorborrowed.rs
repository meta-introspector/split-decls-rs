// Generated macro for CollatorBorrowed (struct)
macro_rules! Depcrate_comparisonCollatorBorrowed {
() => {
// Module: crate::comparison
// Provides: {"CollatorBorrowed"}
// Dependencies: {}
# [doc = " Compares strings according to culturally-relevant ordering,"] # [doc = " borrowed version."] # [derive (Debug)] pub struct CollatorBorrowed < 'a > { special_primaries : & 'a CollationSpecialPrimariesValidated < 'a > , root : & 'a CollationData < 'a > , tailoring : Option < & 'a CollationData < 'a > > , jamo : & 'a CollationJamo < 'a > , diacritics : & 'a CollationDiacritics < 'a > , options : CollatorOptionsBitField , reordering : Option < & 'a CollationReordering < 'a > > , decompositions : & 'a DecompositionData < 'a > , tables : & 'a DecompositionTables < 'a > , lithuanian_dot_above : bool , }
};
}
