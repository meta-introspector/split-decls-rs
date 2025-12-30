// Generated macro for StringRecordsIter (struct)
macro_rules! Depcrate_readerStringRecordsIter {
() => {
// Module: crate::reader
// Provides: {"StringRecordsIter"}
// Dependencies: {}
# [doc = " A borrowed iterator over records as strings."] # [doc = ""] # [doc = " The lifetime parameter `'r` refers to the lifetime of the underlying"] # [doc = " CSV `Reader`."] pub struct StringRecordsIter < 'r , R : 'r > { rdr : & 'r mut Reader < R > , rec : StringRecord , }
};
}
