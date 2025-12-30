// Generated macro for WriteDebugInfo (trait)
macro_rules! Depcrate_debuginfo_objectWriteDebugInfo {
() => {
// Module: crate::debuginfo::object
// Provides: {"WriteDebugInfo"}
// Dependencies: {}
pub (super) trait WriteDebugInfo { type SectionId : Copy ; fn add_debug_section (& mut self , name : SectionId , data : Vec < u8 >) -> Self :: SectionId ; fn add_debug_reloc (& mut self , section_map : & FxHashMap < SectionId , Self :: SectionId > , from : & Self :: SectionId , reloc : & DebugReloc ,) ; }
};
}
