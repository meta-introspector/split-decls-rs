// Generated macro for impl_139 (impl)
macro_rules! Depcrate_readerimpl_139 {
() => {
// Module: crate::reader
// Provides: {"impl_139"}
// Dependencies: {}
impl < 'r , R : io :: Read > ByteRecordsIter < 'r , R > { fn new (rdr : & 'r mut Reader < R >) -> ByteRecordsIter < 'r , R > { ByteRecordsIter { rdr , rec : ByteRecord :: new () } } # [doc = " Return a reference to the underlying CSV reader."] pub fn reader (& self) -> & Reader < R > { self . rdr } # [doc = " Return a mutable reference to the underlying CSV reader."] pub fn reader_mut (& mut self) -> & mut Reader < R > { self . rdr } }
};
}
