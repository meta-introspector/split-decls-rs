// Generated macro for BinaryStream (struct)
macro_rules! Depcrate_readerBinaryStream {
() => {
// Module: crate::reader
// Provides: {"BinaryStream"}
// Dependencies: {}
# [doc = " A direct stream to the underlying [`Reader`]s reader which updates"] # [doc = " [`Reader::buffer_position()`] when read from it."] # [derive (Debug)] # [must_use = "streams do nothing unless read or polled"] pub struct BinaryStream < 'r , R > { inner : & 'r mut R , offset : & 'r mut u64 , }
};
}
