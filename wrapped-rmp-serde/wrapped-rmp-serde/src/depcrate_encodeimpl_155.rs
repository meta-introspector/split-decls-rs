// Generated macro for impl_155 (impl)
macro_rules! Depcrate_encodeimpl_155 {
() => {
// Module: crate::encode
// Provides: {"impl_155"}
// Dependencies: {}
impl < 'a , W : Write + 'a , C : SerializerConfig > SerializeTuple for Tuple < 'a , W , C > { type Ok = () ; type Error = Error ; fn serialize_element < T : ? Sized + Serialize > (& mut self , value : & T) -> Result < () , Self :: Error > { if let Some (buf) = & mut self . buf { if let Ok (byte) = value . serialize (OnlyBytes) { buf . push (byte) ; return Ok (()) ; } else { encode :: write_array_len (& mut self . se . wr , self . len) ? ; for b in buf { b . serialize (& mut * self . se) ? ; } self . buf = None ; } } value . serialize (& mut * self . se) } fn end (self) -> Result < Self :: Ok , Self :: Error > { if let Some (buf) = self . buf { if self . len < 16 && buf . iter () . all (| & b | b < 128) { encode :: write_array_len (& mut self . se . wr , self . len) ? ; } else { encode :: write_bin_len (& mut self . se . wr , self . len) ? ; } self . se . wr . write_all (& buf) . map_err (ValueWriteError :: InvalidDataWrite) ? ; } Ok (()) } }
};
}
