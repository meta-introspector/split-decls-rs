// Generated macro for impl_1294 (impl)
macro_rules! Depcrate_readimpl_1294 {
() => {
// Module: crate::read
// Provides: {"impl_1294"}
// Dependencies: {}
impl SymbolSection { # [doc = " Returns the section index for the section where the symbol is defined."] # [doc = ""] # [doc = " May return `None` if the symbol is not defined in a section."] # [inline] pub fn index (self) -> Option < SectionIndex > { if let SymbolSection :: Section (index) = self { Some (index) } else { None } } }
};
}
