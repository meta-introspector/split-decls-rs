// Generated macro for LocaleSpecificDataHolder (struct)
macro_rules! Depcrate_comparisonLocaleSpecificDataHolder {
() => {
// Module: crate::comparison
// Provides: {"LocaleSpecificDataHolder"}
// Dependencies: {}
# [doc = " Holder struct for payloads that are locale-dependent. (For code"] # [doc = " reuse between owned and borrowed cases.)"] # [derive (Debug)] struct LocaleSpecificDataHolder { tailoring : Option < DataPayload < CollationTailoringV1 > > , diacritics : DataPayload < CollationDiacriticsV1 > , reordering : Option < DataPayload < CollationReorderingV1 > > , merged_options : CollatorOptionsBitField , lithuanian_dot_above : bool , }
};
}
