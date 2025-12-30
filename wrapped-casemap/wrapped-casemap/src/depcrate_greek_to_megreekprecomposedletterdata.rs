// Generated macro for GreekPrecomposedLetterData (enum)
macro_rules! Depcrate_greek_to_meGreekPrecomposedLetterData {
() => {
// Module: crate::greek_to_me
// Provides: {"GreekPrecomposedLetterData"}
// Dependencies: {}
# [doc = " The precomposed letter data stored in the hardcoded data in `mod data`"] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum GreekPrecomposedLetterData { # [doc = " A vowel, with a capitalized base letter, and the diacritics found"] Vowel (GreekVowel , GreekDiacritics) , # [doc = " A consonant or vowel that does not take diacritics"] # [doc = ""] # [doc = " The boolean is true when the consonant is a rho, which is handled specially since"] # [doc = " it can take breathing marks (but is *not* a vowel)"] Consonant (bool) , }
};
}
