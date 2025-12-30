// Generated macro for impl_162 (impl)
macro_rules! Depcrate_deimpl_162 {
() => {
// Module: crate::de
// Provides: {"impl_162"}
// Dependencies: {}
impl < 'de > XmlRead < 'de > for SliceReader < 'de > { fn next (& mut self) -> Result < PayloadEvent < 'de > , DeError > { loop { let event = self . reader . read_event () ? ; if let Some (event) = skip_uninterested (event) { return Ok (event) ; } } } fn read_to_end (& mut self , name : QName) -> Result < () , DeError > { match self . reader . read_to_end (name) { Err (e) => Err (e . into ()) , Ok (_) => Ok (()) , } } fn decoder (& self) -> Decoder { self . reader . decoder () } fn has_nil_attr (& self , start : & BytesStart) -> bool { start . attributes () . has_nil (self . reader . resolver ()) } }
};
}
