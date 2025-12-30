// Generated macro for impl_527 (impl)
macro_rules! Depcrate_repository_tagimpl_527 {
() => {
// Module: crate::repository::tag
// Provides: {"impl_527"}
// Dependencies: {}
impl Version { fn parse (version : & BStr) -> Self { let parts = version . chunk_by (| a , b | a . is_ascii_digit () == b . is_ascii_digit ()) . map (| part | { if let Ok (part) = part . to_str () { part . parse :: < usize > () . map_or_else (| _ | VersionPart :: String (part . into ()) , VersionPart :: Number) } else { VersionPart :: String (part . into ()) } }) . collect () ; Self { parts } } }
};
}
