// Generated macro for InternalTransliterator (enum)
macro_rules! Depcrate_transliterate_transliteratorInternalTransliterator {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"InternalTransliterator"}
// Dependencies: {}
# [derive (Debug)] enum InternalTransliterator { RuleBased (DataPayload < TransliteratorRulesV1 >) , Composing (ComposingNormalizer) , Decomposing (DecomposingNormalizer) , Hex (hardcoded :: HexTransliterator) , Lower (CaseMapper) , Upper (CaseMapper) , Null , Remove , Dyn (Box < dyn CustomTransliterator >) , }
};
}
