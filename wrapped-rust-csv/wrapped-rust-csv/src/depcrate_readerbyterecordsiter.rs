// Generated macro for ByteRecordsIter (struct)
macro_rules! Depcrate_readerByteRecordsIter {
() => {
// Module: crate::reader
// Provides: {"ByteRecordsIter"}
// Dependencies: {}
# [doc = " A borrowed iterator over records as raw bytes."] # [doc = ""] # [doc = " The lifetime parameter `'r` refers to the lifetime of the underlying"] # [doc = " CSV `Reader`."] pub struct ByteRecordsIter < 'r , R : 'r > { rdr : & 'r mut Reader < R > , rec : ByteRecord , }
};
}
