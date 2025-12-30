// Generated macro for impl_198 (impl)
macro_rules! Depcrate_writer_file_into_streamimpl_198 {
() => {
// Module: crate::writer::file::into_stream
// Provides: {"impl_198"}
// Dependencies: {}
impl < const LEN : usize > STREAM_HEADER < LEN > { fn new (offset : u32 , size : u32 , name : & [u8 ; LEN]) -> Self { Self { offset , size , name : * name , } } fn next_offset (& self) -> u32 { self . offset + self . size } }
};
}
