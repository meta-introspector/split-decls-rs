// Generated macro for impl_522 (impl)
macro_rules! Depcrate_se_contentimpl_522 {
() => {
// Module: crate::se::content
// Provides: {"impl_522"}
// Dependencies: {}
impl < 'w , 'i , W : Write > SerializeSeq for Seq < 'w , 'i , W > { type Ok = WriteResult ; type Error = SeError ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { self . last = value . serialize (self . ser . new_seq_element_serializer (self . last . is_text ())) ? ; self . ser . write_indent = self . last . allow_indent () ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (self . last) } }
};
}
