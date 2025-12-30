// Generated macro for impl_1287 (impl)
macro_rules! Depcrate_type_checkimpl_1287 {
() => {
// Module: crate::type_check
// Provides: {"impl_1287"}
// Dependencies: {}
impl Locations { pub fn from_location (& self) -> Option < Location > { match self { Locations :: All (_) => None , Locations :: Single (from_location) => Some (* from_location) , } } # [doc = " Gets a span representing the location."] pub fn span (& self , body : & Body < '_ >) -> Span { match self { Locations :: All (span) => * span , Locations :: Single (l) => body . source_info (* l) . span , } } }
};
}
