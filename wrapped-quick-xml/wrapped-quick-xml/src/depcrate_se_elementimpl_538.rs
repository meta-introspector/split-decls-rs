// Generated macro for impl_538 (impl)
macro_rules! Depcrate_se_elementimpl_538 {
() => {
// Module: crate::se::element
// Provides: {"impl_538"}
// Dependencies: {}
impl < 'w , 'k , W : Write > SerializeSeq for ElementSerializer < 'w , 'k , W > { type Ok = WriteResult ; type Error = SeError ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { value . serialize (ElementSerializer { ser : self . ser . new_seq_element_serializer (true) , key : self . key , }) ? ; self . ser . write_indent = true ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (WriteResult :: Element) } }
};
}
