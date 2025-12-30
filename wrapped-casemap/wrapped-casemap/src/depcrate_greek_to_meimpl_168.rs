// Generated macro for impl_168 (impl)
macro_rules! Depcrate_greek_to_meimpl_168 {
() => {
// Module: crate::greek_to_me
// Provides: {"impl_168"}
// Dependencies: {}
impl TryFrom < char > for GreekVowel { type Error = () ; fn try_from (other : char) -> Result < Self , () > { Ok (match other { 'Α' => GreekVowel :: Α , 'Ε' => GreekVowel :: Ε , 'Η' => GreekVowel :: Η , 'Ι' => GreekVowel :: Ι , 'Ο' => GreekVowel :: Ο , 'Υ' => GreekVowel :: Υ , 'Ω' => GreekVowel :: Ω , 'ϒ' => GreekVowel :: ϒ , _ => return Err (()) , }) } }
};
}
