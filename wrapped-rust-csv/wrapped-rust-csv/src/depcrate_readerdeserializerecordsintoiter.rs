// Generated macro for DeserializeRecordsIntoIter (struct)
macro_rules! Depcrate_readerDeserializeRecordsIntoIter {
() => {
// Module: crate::reader
// Provides: {"DeserializeRecordsIntoIter"}
// Dependencies: {}
# [doc = " An owned iterator over deserialized records."] # [doc = ""] # [doc = " The type parameter `R` refers to the underlying `io::Read` type, and `D`"] # [doc = " refers to the type that this iterator will deserialize a record into."] pub struct DeserializeRecordsIntoIter < R , D > { rdr : Reader < R > , rec : StringRecord , headers : Option < StringRecord > , _priv : PhantomData < D > , }
};
}
