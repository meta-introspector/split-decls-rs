// Generated macro for impl_100 (impl)
macro_rules! Depcrate_queryimpl_100 {
() => {
// Module: crate::query
// Provides: {"impl_100"}
// Dependencies: {}
impl < T : DeserializeOwned > FromQuery for T { type Target = T ; type Error = HistoryError ; fn from_query (query : & str) -> Result < Self :: Target , Self :: Error > { serde_urlencoded :: from_str (query) . map_err (Into :: into) } }
};
}
