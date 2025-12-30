// Generated macro for Source (trait)
macro_rules! Depcrate_sourcesSource {
() => {
// Module: crate::sources
// Provides: {"Source"}
// Dependencies: {}
pub (crate) trait Source { type Item ; type Error ; fn load (& self) -> Result < Self :: Item , Self :: Error > ; }
};
}
