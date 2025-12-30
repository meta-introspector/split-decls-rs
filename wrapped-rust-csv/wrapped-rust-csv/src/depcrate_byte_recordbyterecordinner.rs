// Generated macro for ByteRecordInner (struct)
macro_rules! Depcrate_byte_recordByteRecordInner {
() => {
// Module: crate::byte_record
// Provides: {"ByteRecordInner"}
// Dependencies: {}
# [doc = " The inner portion of a byte record."] # [doc = ""] # [doc = " We use this memory layout so that moving a `ByteRecord` only requires"] # [doc = " moving a single pointer. The optimization is dubious at best, but does"] # [doc = " seem to result in slightly better numbers in microbenchmarks. Methinks this"] # [doc = " may heavily depend on the underlying allocator."] # [derive (Clone , Debug , Eq , PartialEq)] struct ByteRecordInner { # [doc = " The position of this byte record."] pos : Option < Position > , # [doc = " All fields in this record, stored contiguously."] fields : Vec < u8 > , # [doc = " The number of and location of each field in this record."] bounds : Bounds , }
};
}
