// Generated macro for impl_54 (impl)
macro_rules! Depcrate_deimpl_54 {
() => {
// Module: crate::de
// Provides: {"impl_54"}
// Dependencies: {}
impl < T > BorshDeserialize for Option < T > where T : BorshDeserialize , { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let flag : u8 = BorshDeserialize :: deserialize_reader (reader) ? ; if flag == 0 { Ok (None) } else if flag == 1 { Ok (Some (T :: deserialize_reader (reader) ?)) } else { let msg = format ! ("Invalid Option representation: {}. The first byte must be 0 or 1" , flag) ; Err (Error :: new (ErrorKind :: InvalidData , msg)) } } }
};
}
