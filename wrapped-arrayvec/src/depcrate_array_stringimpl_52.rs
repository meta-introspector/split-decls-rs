// Generated macro for impl_52 (impl)
macro_rules! Depcrate_array_stringimpl_52 {
() => {
// Module: crate::array_string
// Provides: {"impl_52"}
// Dependencies: {}
# [cfg (feature = "borsh")] # [doc = " Requires crate feature `\"borsh\"`"] impl < const CAP : usize > borsh :: BorshDeserialize for ArrayString < CAP > { fn deserialize_reader < R : borsh :: io :: Read > (reader : & mut R) -> borsh :: io :: Result < Self > { let len = < u32 as borsh :: BorshDeserialize > :: deserialize_reader (reader) ? as usize ; if len > CAP { return Err (borsh :: io :: Error :: new (borsh :: io :: ErrorKind :: InvalidData , format ! ("Expected a string no more than {} bytes long" , CAP) ,)) ; } let mut buf = [0u8 ; CAP] ; let buf = & mut buf [.. len] ; reader . read_exact (buf) ? ; let s = str :: from_utf8 (& buf) . map_err (| err | { borsh :: io :: Error :: new (borsh :: io :: ErrorKind :: InvalidData , err . to_string ()) }) ? ; Ok (Self :: from (s) . unwrap ()) } }
};
}
