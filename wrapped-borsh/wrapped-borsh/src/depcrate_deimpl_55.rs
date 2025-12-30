// Generated macro for impl_55 (impl)
macro_rules! Depcrate_deimpl_55 {
() => {
// Module: crate::de
// Provides: {"impl_55"}
// Dependencies: {}
impl < T , E > BorshDeserialize for core :: result :: Result < T , E > where T : BorshDeserialize , E : BorshDeserialize , { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let flag : u8 = BorshDeserialize :: deserialize_reader (reader) ? ; if flag == 0 { Ok (Err (E :: deserialize_reader (reader) ?)) } else if flag == 1 { Ok (Ok (T :: deserialize_reader (reader) ?)) } else { let msg = format ! ("Invalid Result representation: {}. The first byte must be 0 or 1" , flag) ; Err (Error :: new (ErrorKind :: InvalidData , msg)) } } }
};
}
