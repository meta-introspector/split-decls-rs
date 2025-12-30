// Generated macro for impl_542 (impl)
macro_rules! Depcrate_se_elementimpl_542 {
() => {
// Module: crate::se::element
// Provides: {"impl_542"}
// Dependencies: {}
impl < 'w , 'k , W : Write > SerializeTupleVariant for Tuple < 'w , 'k , W > { type Ok = WriteResult ; type Error = SeError ; # [inline] fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { match self { Self :: Element (ser) => SerializeTuple :: serialize_element (ser , value) , Self :: Text (ser) => SerializeTuple :: serialize_element (ser , value) , } } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { match self { Self :: Element (ser) => SerializeTuple :: end (ser) , Self :: Text (ser) => SerializeTuple :: end (ser) . map (| _ | WriteResult :: SensitiveText) , } } }
};
}
