// Generated macro for DeserializeRecordsIter (struct)
macro_rules! Depcrate_readerDeserializeRecordsIter {
() => {
// Module: crate::reader
// Provides: {"DeserializeRecordsIter"}
// Dependencies: {}
# [doc = " A borrowed iterator over deserialized records."] # [doc = ""] # [doc = " The lifetime parameter `'r` refers to the lifetime of the underlying"] # [doc = " CSV `Reader`. The type parameter `R` refers to the underlying `io::Read`"] # [doc = " type, and `D` refers to the type that this iterator will deserialize a"] # [doc = " record into."] pub struct DeserializeRecordsIter < 'r , R : 'r , D > { rdr : & 'r mut Reader < R > , rec : StringRecord , headers : Option < StringRecord > , _priv : PhantomData < D > , }
};
}
