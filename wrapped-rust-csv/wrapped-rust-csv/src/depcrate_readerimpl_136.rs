// Generated macro for impl_136 (impl)
macro_rules! Depcrate_readerimpl_136 {
() => {
// Module: crate::reader
// Provides: {"impl_136"}
// Dependencies: {}
impl < R : io :: Read > ByteRecordsIntoIter < R > { fn new (rdr : Reader < R >) -> ByteRecordsIntoIter < R > { ByteRecordsIntoIter { rdr , rec : ByteRecord :: new () } } # [doc = " Return a reference to the underlying CSV reader."] pub fn reader (& self) -> & Reader < R > { & self . rdr } # [doc = " Return a mutable reference to the underlying CSV reader."] pub fn reader_mut (& mut self) -> & mut Reader < R > { & mut self . rdr } # [doc = " Drop this iterator and return the underlying CSV reader."] pub fn into_reader (self) -> Reader < R > { self . rdr } }
};
}
