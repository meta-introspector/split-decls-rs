// Generated macro for impl_386 (impl)
macro_rules! Depcrate_provider_fields_lengthimpl_386 {
() => {
// Module: crate::provider::fields::length
// Provides: {"impl_386"}
// Dependencies: {}
impl TryFrom < u8 > for FieldNumericOverrides { type Error = LengthError ; fn try_from (other : u8) -> Result < Self , LengthError > { Ok (match other { 0 => Self :: Hanidec , 1 => Self :: Hanidays , 2 => Self :: Hebr , 3 => Self :: Romanlow , 4 => Self :: Jpnyear , _ => return Err (LengthError :: InvalidLength) , }) } }
};
}
