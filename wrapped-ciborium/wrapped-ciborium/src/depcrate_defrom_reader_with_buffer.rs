// Generated macro for from_reader_with_buffer (function)
macro_rules! Depcrate_defrom_reader_with_buffer {
() => {
// Module: crate::de
// Provides: {"from_reader_with_buffer"}
// Dependencies: {}
# [doc = " Deserializes as CBOR from a type with [`impl"] # [doc = " ciborium_io::Read`](ciborium_io::Read), using a caller-specific buffer as a"] # [doc = " temporary scratch space."] # [inline] pub fn from_reader_with_buffer < T : de :: DeserializeOwned , R : Read > (reader : R , scratch_buffer : & mut [u8] ,) -> Result < T , Error < R :: Error > > where R :: Error : core :: fmt :: Debug , { let mut reader = Deserializer { decoder : reader . into () , scratch : scratch_buffer , recurse : 256 , } ; T :: deserialize (& mut reader) }
};
}
