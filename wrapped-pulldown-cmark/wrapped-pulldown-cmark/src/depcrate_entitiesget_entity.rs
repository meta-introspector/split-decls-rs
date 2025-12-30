// Generated macro for get_entity (function)
macro_rules! Depcrate_entitiesget_entity {
() => {
// Module: crate::entities
// Provides: {"get_entity"}
// Dependencies: {}
pub (crate) fn get_entity (bytes : & [u8]) -> Option < & 'static str > { ENTITIES . binary_search_by_key (& bytes , | & (key , _value) | key) . ok () . map (| i | ENTITIES [i] . 1) }
};
}
