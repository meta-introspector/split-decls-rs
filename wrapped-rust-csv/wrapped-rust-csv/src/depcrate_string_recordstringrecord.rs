// Generated macro for StringRecord (struct)
macro_rules! Depcrate_string_recordStringRecord {
() => {
// Module: crate::string_record
// Provides: {"StringRecord"}
// Dependencies: {}
# [doc = " A single CSV record stored as valid UTF-8 bytes."] # [doc = ""] # [doc = " A string record permits reading or writing CSV rows that are valid UTF-8."] # [doc = " If string records are used to read CSV data that is not valid UTF-8, then"] # [doc = " the CSV reader will return an invalid UTF-8 error. If you do need to read"] # [doc = " possibly invalid UTF-8 data, then you should prefer using a"] # [doc = " [`ByteRecord`](struct.ByteRecord.html),"] # [doc = " since it makes no assumptions about UTF-8."] # [doc = ""] # [doc = " If you are using the Serde (de)serialization APIs, then you probably never"] # [doc = " need to interact with a `ByteRecord` or a `StringRecord`. However, there"] # [doc = " are some circumstances in which you might need to use a raw record type"] # [doc = " while still using Serde. For example, if you need to deserialize possibly"] # [doc = " invalid UTF-8 fields, then you'll need to first read your record into a"] # [doc = " `ByteRecord`, and then use `ByteRecord::deserialize` to run Serde. Another"] # [doc = " reason for using the raw record deserialization APIs is if you're using"] # [doc = " Serde to read into borrowed data such as a `&'a str` or a `&'a [u8]`."] # [doc = ""] # [doc = " Two `StringRecord`s are compared on the basis of their field data. Any"] # [doc = " position information associated with the records is ignored."] # [derive (Clone , Eq)] pub struct StringRecord (ByteRecord) ;
};
}
