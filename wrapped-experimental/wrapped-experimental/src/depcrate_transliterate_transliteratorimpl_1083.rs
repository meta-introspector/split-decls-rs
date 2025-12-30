// Generated macro for impl_1083 (impl)
macro_rules! Depcrate_transliterate_transliteratorimpl_1083 {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"impl_1083"}
// Dependencies: {}
# [cfg (feature = "compiled_data")] impl Default for TransliteratorBuilder { fn default () -> Self { Self { env : LiteMap :: from_iter ([("any-remove" . into () , InternalTransliterator :: Remove) , ("any-null" . into () , InternalTransliterator :: Null) ,]) , transliterator : DataPayload :: from_owned (RuleBasedTransliterator { visibility : false , variable_table : Default :: default () , filter : CodePointInversionList :: all () , id_group_list : Default :: default () , rule_group_list : Default :: default () , }) , } } }
};
}
