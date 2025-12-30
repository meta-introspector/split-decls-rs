// Generated macro for DeByteRecord (struct)
macro_rules! Depcrate_deserializerDeByteRecord {
() => {
// Module: crate::deserializer
// Provides: {"DeByteRecord"}
// Dependencies: {}
struct DeByteRecord < 'r > { it : iter :: Peekable < ByteRecordIter < 'r > > , headers : Option < ByteRecordIter < 'r > > , field : u64 , }
};
}
