// Generated macro for from_reader_with_recursion_limit (function)
macro_rules! Depcrate_defrom_reader_with_recursion_limit {
() => {
// Module: crate::de
// Provides: {"from_reader_with_recursion_limit"}
// Dependencies: {}
# [doc = " Deserializes as CBOR from a type with [`impl ciborium_io::Read`](ciborium_io::Read), with"] # [doc = " a specified maximum recursion limit.  Inputs that are nested beyond the specified limit"] # [doc = " will result in [`Error::RecursionLimitExceeded`] ."] # [doc = ""] # [doc = " Set a high recursion limit at your own risk (of stack exhaustion)!"] # [inline] pub fn from_reader_with_recursion_limit < T : de :: DeserializeOwned , R : Read > (reader : R , recurse_limit : usize ,) -> Result < T , Error < R :: Error > > where R :: Error : core :: fmt :: Debug , { let mut scratch = [0 ; 4096] ; let mut reader = Deserializer { decoder : reader . into () , scratch : & mut scratch , recurse : recurse_limit , } ; T :: deserialize (& mut reader) }
};
}
