// Generated macro for impl_124 (impl)
macro_rules! Depcrate_readerimpl_124 {
() => {
// Module: crate::reader
// Provides: {"impl_124"}
// Dependencies: {}
impl < R : io :: Read , D : DeserializeOwned > DeserializeRecordsIntoIter < R , D > { fn new (mut rdr : Reader < R >) -> DeserializeRecordsIntoIter < R , D > { let headers = if ! rdr . state . has_headers { None } else { rdr . headers () . ok () . cloned () } ; DeserializeRecordsIntoIter { rdr , rec : StringRecord :: new () , headers , _priv : PhantomData , } } # [doc = " Return a reference to the underlying CSV reader."] pub fn reader (& self) -> & Reader < R > { & self . rdr } # [doc = " Return a mutable reference to the underlying CSV reader."] pub fn reader_mut (& mut self) -> & mut Reader < R > { & mut self . rdr } # [doc = " Drop this iterator and return the underlying CSV reader."] pub fn into_reader (self) -> Reader < R > { self . rdr } }
};
}
