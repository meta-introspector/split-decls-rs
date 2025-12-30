// Generated macro for impl_169 (impl)
macro_rules! Depcrate_encodeimpl_169 {
() => {
// Module: crate::encode
// Provides: {"impl_169"}
// Dependencies: {}
impl < 'a , W : Write + 'a , C : SerializerConfig > SerializeMap for MaybeUnknownLengthCompound < 'a , W , C > { type Ok = () ; type Error = Error ; fn serialize_key < T : ? Sized + Serialize > (& mut self , key : & T) -> Result < () , Self :: Error > { < Self as SerializeSeq > :: serialize_element (self , key) } fn serialize_value < T : ? Sized + Serialize > (& mut self , value : & T) -> Result < () , Self :: Error > { < Self as SerializeSeq > :: serialize_element (self , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { if let Some (compound) = self . compound { encode :: write_map_len (& mut self . se . wr , compound . elem_count / 2) ? ; self . se . wr . write_all (& compound . se . into_inner ()) . map_err (ValueWriteError :: InvalidDataWrite) ? ; } Ok (()) } }
};
}
