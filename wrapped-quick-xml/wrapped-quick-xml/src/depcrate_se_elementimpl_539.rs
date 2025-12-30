// Generated macro for impl_539 (impl)
macro_rules! Depcrate_se_elementimpl_539 {
() => {
// Module: crate::se::element
// Provides: {"impl_539"}
// Dependencies: {}
impl < 'w , 'k , W : Write > SerializeTuple for ElementSerializer < 'w , 'k , W > { type Ok = WriteResult ; type Error = SeError ; # [inline] fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { SerializeSeq :: serialize_element (self , value) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { SerializeSeq :: end (self) } }
};
}
