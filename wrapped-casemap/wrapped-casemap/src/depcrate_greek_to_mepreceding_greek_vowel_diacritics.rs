// Generated macro for preceding_greek_vowel_diacritics (function)
macro_rules! Depcrate_greek_to_mepreceding_greek_vowel_diacritics {
() => {
// Module: crate::greek_to_me
// Provides: {"preceding_greek_vowel_diacritics"}
// Dependencies: {}
# [doc = " Returns diacritic information for the combining character sequence preceding the current character"] # [doc = " if it that preceding combining character sequence is a greek vowel."] pub (crate) fn preceding_greek_vowel_diacritics (context_before : & str ,) -> Option < GreekCombiningCharacterSequenceDiacritics > { let mut combining : GreekDiacritics = Default :: default () ; for c in context_before . chars () . rev () { match c { diacritics ! (ACCENTS) => combining . accented = true , diacritics ! (DIALYTIKA_TONOS) => { combining . dialytika = true ; combining . accented = true ; } diacritics ! (DIALYTIKA) => combining . dialytika = true , diacritics ! (BREATHING_AND_LENGTH) => continue , _ => { let data = get_data (c) ; if let Some (GreekPrecomposedLetterData :: Vowel (_vowel , diacritics)) = data { return Some (GreekCombiningCharacterSequenceDiacritics { precomposed : diacritics , combining , }) ; } else { return None ; } } } } None }
};
}
