// Generated macro for Collator (struct)
macro_rules! Depcrate_comparisonCollator {
() => {
// Module: crate::comparison
// Provides: {"Collator"}
// Dependencies: {}
# [doc = " Compares strings according to culturally-relevant ordering."] # [derive (Debug)] pub struct Collator { special_primaries : DataPayload < ErasedMarker < CollationSpecialPrimariesValidated < 'static > > > , root : DataPayload < CollationRootV1 > , tailoring : Option < DataPayload < CollationTailoringV1 > > , jamo : DataPayload < CollationJamoV1 > , diacritics : DataPayload < CollationDiacriticsV1 > , options : CollatorOptionsBitField , reordering : Option < DataPayload < CollationReorderingV1 > > , decompositions : DataPayload < NormalizerNfdDataV1 > , tables : DataPayload < NormalizerNfdTablesV1 > , lithuanian_dot_above : bool , }
};
}
