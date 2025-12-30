// Generated macro for impl_1509 (impl)
macro_rules! Depcrate_writeimpl_1509 {
() => {
// Module: crate::write
// Provides: {"impl_1509"}
// Dependencies: {}
impl SymbolSection { # [doc = " Returns the section id for the section where the symbol is defined."] # [doc = ""] # [doc = " May return `None` if the symbol is not defined in a section."] # [inline] pub fn id (self) -> Option < SectionId > { if let SymbolSection :: Section (id) = self { Some (id) } else { None } } }
};
}
