// Generated macro for BorshDeserialize (trait)
macro_rules! Depcrate_deBorshDeserialize {
() => {
// Module: crate::de
// Provides: {"BorshDeserialize"}
// Dependencies: {}
# [doc = " A data-structure that can be de-serialized from binary format by NBOR."] pub trait BorshDeserialize : Sized { # [doc = " Deserializes this instance from a given slice of bytes."] # [doc = " Updates the buffer to point at the remaining bytes."] fn deserialize (buf : & mut & [u8]) -> Result < Self > { Self :: deserialize_reader (& mut * buf) } fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > ; # [doc = " Deserialize this instance from a slice of bytes."] fn try_from_slice (v : & [u8]) -> Result < Self > { let mut v_mut = v ; let result = Self :: deserialize (& mut v_mut) ? ; if ! v_mut . is_empty () { return Err (Error :: new (ErrorKind :: InvalidData , ERROR_NOT_ALL_BYTES_READ)) ; } Ok (result) } fn try_from_reader < R : Read > (reader : & mut R) -> Result < Self > { let result = Self :: deserialize_reader (reader) ? ; let mut buf = [0u8 ; 1] ; match reader . read_exact (& mut buf) { Err (f) if f . kind () == ErrorKind :: UnexpectedEof => Ok (result) , _ => Err (Error :: new (ErrorKind :: InvalidData , ERROR_NOT_ALL_BYTES_READ)) , } } # [inline] # [doc (hidden)] fn vec_from_reader < R : Read > (len : u32 , reader : & mut R) -> Result < Option < Vec < Self > > > { let _ = len ; let _ = reader ; Ok (None) } # [inline] # [doc (hidden)] fn array_from_reader < R : Read , const N : usize > (reader : & mut R) -> Result < Option < [Self ; N] > > { let _ = reader ; Ok (None) } }
};
}
