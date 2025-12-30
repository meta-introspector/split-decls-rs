// Generated macro for impl_99 (impl)
macro_rules! Depcrate_queryimpl_99 {
() => {
// Module: crate::query
// Provides: {"impl_99"}
// Dependencies: {}
impl < T : Serialize > ToQuery for T { type Error = HistoryError ; fn to_query (& self) -> Result < Cow < '_ , str > , Self :: Error > { serde_urlencoded :: to_string (self) . map (Into :: into) . map_err (Into :: into) } }
};
}
