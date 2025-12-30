// Generated macro for impl_1008 (impl)
macro_rules! Depcrate_read_pe_importimpl_1008 {
() => {
// Module: crate::read::pe::import
// Provides: {"impl_1008"}
// Dependencies: {}
impl < 'data > ImportDescriptorIterator < 'data > { # [doc = " Return the next descriptor."] # [doc = ""] # [doc = " Returns `Ok(None)` when a null descriptor is found."] pub fn next (& mut self) -> Result < Option < & 'data pe :: ImageImportDescriptor > > { if self . null { return Ok (None) ; } let result = self . data . read :: < pe :: ImageImportDescriptor > () . read_error ("Missing PE null import descriptor") ; match result { Ok (import_desc) => { if import_desc . is_null () { self . null = true ; Ok (None) } else { Ok (Some (import_desc)) } } Err (e) => { self . null = true ; Err (e) } } } }
};
}
