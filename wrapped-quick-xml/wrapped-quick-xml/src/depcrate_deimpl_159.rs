// Generated macro for impl_159 (impl)
macro_rules! Depcrate_deimpl_159 {
() => {
// Module: crate::de
// Provides: {"impl_159"}
// Dependencies: {}
impl < 'i , R : BufRead > XmlRead < 'i > for IoReader < R > { fn next (& mut self) -> Result < PayloadEvent < 'static > , DeError > { loop { self . buf . clear () ; let event = self . reader . read_event_into (& mut self . buf) ? ; if let Some (event) = skip_uninterested (event) { return Ok (event . into_owned ()) ; } } } fn read_to_end (& mut self , name : QName) -> Result < () , DeError > { match self . reader . read_to_end_into (name , & mut self . buf) { Err (e) => Err (e . into ()) , Ok (_) => Ok (()) , } } fn decoder (& self) -> Decoder { self . reader . decoder () } fn has_nil_attr (& self , start : & BytesStart) -> bool { start . attributes () . has_nil (self . reader . resolver ()) } }
};
}
