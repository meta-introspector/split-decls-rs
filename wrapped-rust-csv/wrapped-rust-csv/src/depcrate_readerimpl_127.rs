// Generated macro for impl_127 (impl)
macro_rules! Depcrate_readerimpl_127 {
() => {
// Module: crate::reader
// Provides: {"impl_127"}
// Dependencies: {}
impl < 'r , R : io :: Read , D : DeserializeOwned > DeserializeRecordsIter < 'r , R , D > { fn new (rdr : & 'r mut Reader < R >) -> DeserializeRecordsIter < 'r , R , D > { let headers = if ! rdr . state . has_headers { None } else { rdr . headers () . ok () . cloned () } ; DeserializeRecordsIter { rdr , rec : StringRecord :: new () , headers , _priv : PhantomData , } } # [doc = " Return a reference to the underlying CSV reader."] pub fn reader (& self) -> & Reader < R > { self . rdr } # [doc = " Return a mutable reference to the underlying CSV reader."] pub fn reader_mut (& mut self) -> & mut Reader < R > { self . rdr } }
};
}
