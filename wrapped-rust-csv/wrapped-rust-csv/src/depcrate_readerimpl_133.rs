// Generated macro for impl_133 (impl)
macro_rules! Depcrate_readerimpl_133 {
() => {
// Module: crate::reader
// Provides: {"impl_133"}
// Dependencies: {}
impl < 'r , R : io :: Read > StringRecordsIter < 'r , R > { fn new (rdr : & 'r mut Reader < R >) -> StringRecordsIter < 'r , R > { StringRecordsIter { rdr , rec : StringRecord :: new () } } # [doc = " Return a reference to the underlying CSV reader."] pub fn reader (& self) -> & Reader < R > { self . rdr } # [doc = " Return a mutable reference to the underlying CSV reader."] pub fn reader_mut (& mut self) -> & mut Reader < R > { self . rdr } }
};
}
