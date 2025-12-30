// Generated macro for from_reader (function)
macro_rules! Depcrate_defrom_reader {
() => {
// Module: crate::de
// Provides: {"from_reader"}
// Dependencies: {}
# [doc = " Deserializes as CBOR from a type with [`impl"] # [doc = " ciborium_io::Read`](ciborium_io::Read) using a 4KB buffer on the stack."] # [doc = ""] # [doc = " If you want to deserialize faster at the cost of more memory, consider using"] # [doc = " [`from_reader_with_buffer`](from_reader_with_buffer) with a larger buffer,"] # [doc = " for example 64KB."] # [inline] pub fn from_reader < T : de :: DeserializeOwned , R : Read > (reader : R) -> Result < T , Error < R :: Error > > where R :: Error : core :: fmt :: Debug , { let mut scratch = [0 ; 4096] ; from_reader_with_buffer (reader , & mut scratch) }
};
}
