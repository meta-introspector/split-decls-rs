// Generated macro for PackedGreekPrecomposedLetterData (struct)
macro_rules! Depcrate_greek_to_mePackedGreekPrecomposedLetterData {
() => {
// Module: crate::greek_to_me
// Provides: {"PackedGreekPrecomposedLetterData"}
// Dependencies: {}
# [doc = " A packed representation of [`GreekPrecomposedLetterData`]"] # [doc = ""] # [doc = " Bit layout:"] # [doc = ""] # [doc = " ```text"] # [doc = "   7       6   5   4     3   2   1       0"] # [doc = " discr=0 | [diacritics]  | [vowel            ]  "] # [doc = " discr=1 | [  unused = 0     ]      | [is_rho]"] # [doc = " ```"] # [doc = ""] # [doc = " Bit 7 is the discriminant. if 0, it is a vowel, else, it is a consonant."] # [doc = " If the whole thing is a zero then it is assumed to be an empty entry."] # [doc = ""] # [doc = " In the vowel case, the next three bits are the next three elements of GreekDiacritics,"] # [doc = " in order (accented, dialytika, ypogegrammeni), and the four bits after that identify"] # [doc = " a GreekVowel value."] # [doc = ""] # [doc = " In the consonant case, the remaining seven bits identify a GreekConsonant value."] # [derive (Debug , Clone , Copy)] pub struct PackedGreekPrecomposedLetterData (pub u8) ;
};
}
