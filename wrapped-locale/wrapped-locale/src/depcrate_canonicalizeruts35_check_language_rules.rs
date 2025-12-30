// Generated macro for uts35_check_language_rules (function)
macro_rules! Depcrate_canonicalizeruts35_check_language_rules {
() => {
// Module: crate::canonicalizer
// Provides: {"uts35_check_language_rules"}
// Dependencies: {}
# [inline] fn uts35_check_language_rules (langid : & mut LanguageIdentifier , alias_data : & DataPayload < LocaleAliasesV1 > ,) -> TransformResult { if ! langid . language . is_unknown () { let lang : TinyAsciiStr < 3 > = langid . language . into () ; let replacement = if lang . len () == 2 { alias_data . get () . language_len2 . get (& lang . resize () . to_unvalidated ()) } else { alias_data . get () . language_len3 . get (& lang . to_unvalidated ()) } ; if let Some (replacement) = replacement { if let Ok (new_langid) = replacement . parse () { uts35_replacement :: < core :: iter :: Empty < & str > > (langid , true , false , false , None , & new_langid ,) ; return TransformResult :: Modified ; } } } TransformResult :: Unmodified }
};
}
