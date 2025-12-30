// Generated macro for impl_69 (impl)
macro_rules! Depcrate_de_resolverimpl_69 {
() => {
// Module: crate::de::resolver
// Provides: {"impl_69"}
// Dependencies: {}
impl EntityResolver for PredefinedEntityResolver { type Error = Infallible ; # [inline] fn capture (& mut self , _doctype : BytesText) -> Result < () , Self :: Error > { Ok (()) } # [inline] fn resolve (& self , entity : & str) -> Option < & str > { resolve_predefined_entity (entity) } }
};
}
