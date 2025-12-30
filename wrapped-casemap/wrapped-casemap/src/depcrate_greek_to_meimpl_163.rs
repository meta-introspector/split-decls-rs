// Generated macro for impl_163 (impl)
macro_rules! Depcrate_greek_to_meimpl_163 {
() => {
// Module: crate::greek_to_me
// Provides: {"impl_163"}
// Dependencies: {}
impl From < GreekPrecomposedLetterData > for PackedGreekPrecomposedLetterData { fn from (other : GreekPrecomposedLetterData) -> Self { match other { GreekPrecomposedLetterData :: Vowel (vowel , diacritics) => { let mut bits = 0 ; if diacritics . accented { bits |= 0x40 ; } if diacritics . dialytika { bits |= 0x20 ; } if diacritics . ypogegrammeni { bits |= 0x10 ; } bits |= vowel as u8 ; PackedGreekPrecomposedLetterData (bits) } GreekPrecomposedLetterData :: Consonant (is_rho) => { PackedGreekPrecomposedLetterData (0x80 + is_rho as u8) } } } }
};
}
