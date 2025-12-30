// Generated macro for ByteRecordIter (struct)
macro_rules! Depcrate_byte_recordByteRecordIter {
() => {
// Module: crate::byte_record
// Provides: {"ByteRecordIter"}
// Dependencies: {}
# [doc = " A double-ended iterator over the fields in a byte record."] # [doc = ""] # [doc = " The `'r` lifetime variable refers to the lifetime of the `ByteRecord` that"] # [doc = " is being iterated over."] # [derive (Clone)] pub struct ByteRecordIter < 'r > { # [doc = " The record we are iterating over."] r : & 'r ByteRecord , # [doc = " The starting index of the previous field. (For reverse iteration.)"] last_start : usize , # [doc = " The ending index of the previous field. (For forward iteration.)"] last_end : usize , # [doc = " The index of forward iteration."] i_forward : usize , # [doc = " The index of reverse iteration."] i_reverse : usize , }
};
}
