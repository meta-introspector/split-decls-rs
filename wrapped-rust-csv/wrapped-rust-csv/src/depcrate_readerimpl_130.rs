// Generated macro for impl_130 (impl)
macro_rules! Depcrate_readerimpl_130 {
() => {
// Module: crate::reader
// Provides: {"impl_130"}
// Dependencies: {}
impl < R : io :: Read > StringRecordsIntoIter < R > { fn new (rdr : Reader < R >) -> StringRecordsIntoIter < R > { StringRecordsIntoIter { rdr , rec : StringRecord :: new () } } # [doc = " Return a reference to the underlying CSV reader."] pub fn reader (& self) -> & Reader < R > { & self . rdr } # [doc = " Return a mutable reference to the underlying CSV reader."] pub fn reader_mut (& mut self) -> & mut Reader < R > { & mut self . rdr } # [doc = " Drop this iterator and return the underlying CSV reader."] pub fn into_reader (self) -> Reader < R > { self . rdr } }
};
}
