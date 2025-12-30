// Generated macro for BinaryReader (struct)
macro_rules! Depcrate_stream_binary_readerBinaryReader {
() => {
// Module: crate::stream::binary_reader
// Provides: {"BinaryReader"}
// Dependencies: {}
pub struct BinaryReader < R > { stack : Vec < StackItem > , object_offsets : Vec < u64 > , object_on_stack : Vec < bool > , reader : PosReader < R > , ref_size : u8 , root_object : u64 , trailer_start_offset : u64 , }
};
}
