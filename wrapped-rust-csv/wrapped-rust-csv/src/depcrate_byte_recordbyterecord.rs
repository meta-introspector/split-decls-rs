// Generated macro for ByteRecord (struct)
macro_rules! Depcrate_byte_recordByteRecord {
() => {
// Module: crate::byte_record
// Provides: {"ByteRecord"}
// Dependencies: {}
# [doc = " A single CSV record stored as raw bytes."] # [doc = ""] # [doc = " A byte record permits reading or writing CSV rows that are not UTF-8."] # [doc = " In general, you should prefer using a"] # [doc = " [`StringRecord`](struct.StringRecord.html)"] # [doc = " since it is more ergonomic, but a `ByteRecord` is provided in case you need"] # [doc = " it."] # [doc = ""] # [doc = " If you are using the Serde (de)serialization APIs, then you probably never"] # [doc = " need to interact with a `ByteRecord` or a `StringRecord`. However, there"] # [doc = " are some circumstances in which you might need to use a raw record type"] # [doc = " while still using Serde. For example, if you need to deserialize possibly"] # [doc = " invalid UTF-8 fields, then you'll need to first read your record into a"] # [doc = " `ByteRecord`, and then use `ByteRecord::deserialize` to run Serde. Another"] # [doc = " reason for using the raw record deserialization APIs is if you're using"] # [doc = " Serde to read into borrowed data such as a `&'a str` or a `&'a [u8]`."] # [doc = ""] # [doc = " Two `ByteRecord`s are compared on the basis of their field data. Any"] # [doc = " position information associated with the records is ignored."] # [derive (Clone , Eq)] pub struct ByteRecord (Box < ByteRecordInner >) ;
};
}
