// Generated macro for impl_659 (impl)
macro_rules! Depcrate_read_elf_attributesimpl_659 {
() => {
// Module: crate::read::elf::attributes
// Provides: {"impl_659"}
// Dependencies: {}
impl < 'data > AttributeIndexIterator < 'data > { # [doc = " Parse the next index."] pub fn next (& mut self) -> Result < Option < u32 > > { if self . data . is_empty () { return Ok (None) ; } let result = self . parse () . map (Some) ; if result . is_err () { self . data = Bytes (& []) ; } result } fn parse (& mut self) -> Result < u32 > { let err = "Invalid ELF attribute index" ; self . data . read_uleb128 () . read_error (err) ? . try_into () . map_err (| _ | ()) . read_error (err) } }
};
}
