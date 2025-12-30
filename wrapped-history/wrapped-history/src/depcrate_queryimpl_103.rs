// Generated macro for impl_103 (impl)
macro_rules! Depcrate_queryimpl_103 {
() => {
// Module: crate::query
// Provides: {"impl_103"}
// Dependencies: {}
impl < T : for < 'a > From < & 'a str > > FromQuery for Raw < T > { type Target = T ; type Error = Infallible ; fn from_query (query : & str) -> Result < Self :: Target , Self :: Error > { Ok (query . into ()) } }
};
}
