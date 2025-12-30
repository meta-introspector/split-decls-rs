// Generated macro for impl_168 (impl)
macro_rules! Depcrate_encodeimpl_168 {
() => {
// Module: crate::encode
// Provides: {"impl_168"}
// Dependencies: {}
impl < 'a , W : Write + 'a , C : SerializerConfig > SerializeSeq for MaybeUnknownLengthCompound < 'a , W , C > { type Ok = () ; type Error = Error ; fn serialize_element < T : ? Sized + Serialize > (& mut self , value : & T) -> Result < () , Self :: Error > { match self . compound . as_mut () { None => value . serialize (& mut * self . se) , Some (buf) => { value . serialize (& mut buf . se) ? ; buf . elem_count += 1 ; Ok (()) } } } fn end (self) -> Result < Self :: Ok , Self :: Error > { if let Some (compound) = self . compound { encode :: write_array_len (& mut self . se . wr , compound . elem_count) ? ; self . se . wr . write_all (& compound . se . into_inner ()) . map_err (ValueWriteError :: InvalidDataWrite) ? ; } Ok (()) } }
};
}
