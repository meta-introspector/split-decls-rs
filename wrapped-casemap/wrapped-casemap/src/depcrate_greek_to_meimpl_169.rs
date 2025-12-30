// Generated macro for impl_169 (impl)
macro_rules! Depcrate_greek_to_meimpl_169 {
() => {
// Module: crate::greek_to_me
// Provides: {"impl_169"}
// Dependencies: {}
impl TryFrom < u8 > for GreekVowel { type Error = () ; fn try_from (other : u8) -> Result < Self , () > { Ok (match other { 1 => Self :: Α , 2 => Self :: Ε , 3 => Self :: Η , 4 => Self :: Ι , 5 => Self :: Ο , 6 => Self :: Υ , 7 => Self :: Ω , 8 => Self :: ϒ , _ => return Err (()) , }) } }
};
}
