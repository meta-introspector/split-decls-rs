// Generated macro for impl_53 (impl)
macro_rules! Depcrate_deimpl_53 {
() => {
// Module: crate::de
// Provides: {"impl_53"}
// Dependencies: {}
impl BorshDeserialize for bool { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let b : u8 = BorshDeserialize :: deserialize_reader (reader) ? ; if b == 0 { Ok (false) } else if b == 1 { Ok (true) } else { let msg = format ! ("Invalid bool representation: {}" , b) ; Err (Error :: new (ErrorKind :: InvalidData , msg)) } } }
};
}
