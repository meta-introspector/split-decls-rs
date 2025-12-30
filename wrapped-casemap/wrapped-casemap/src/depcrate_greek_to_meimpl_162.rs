// Generated macro for impl_162 (impl)
macro_rules! Depcrate_greek_to_meimpl_162 {
() => {
// Module: crate::greek_to_me
// Provides: {"impl_162"}
// Dependencies: {}
impl TryFrom < PackedGreekPrecomposedLetterData > for GreekPrecomposedLetterData { type Error = () ; fn try_from (other : PackedGreekPrecomposedLetterData) -> Result < GreekPrecomposedLetterData , () > { if other . 0 == 0 { return Err (()) ; } if other . 0 & 0x80 == 0 { let diacritics = GreekDiacritics { accented : other . 0 & 0x40 != 0 , dialytika : other . 0 & 0x20 != 0 , ypogegrammeni : other . 0 & 0x10 != 0 , } ; let vowel = GreekVowel :: try_from (other . 0 & 0b1111) ; debug_assert ! (vowel . is_ok ()) ; let vowel = vowel . unwrap_or (GreekVowel :: Α) ; Ok (GreekPrecomposedLetterData :: Vowel (vowel , diacritics)) } else { Ok (GreekPrecomposedLetterData :: Consonant (other . 0 == 0x81)) } } }
};
}
