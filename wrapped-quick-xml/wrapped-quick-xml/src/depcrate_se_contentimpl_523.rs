// Generated macro for impl_523 (impl)
macro_rules! Depcrate_se_contentimpl_523 {
() => {
// Module: crate::se::content
// Provides: {"impl_523"}
// Dependencies: {}
impl < 'w , 'i , W : Write > SerializeTuple for Seq < 'w , 'i , W > { type Ok = WriteResult ; type Error = SeError ; # [inline] fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { SerializeSeq :: serialize_element (self , value) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { SerializeSeq :: end (self) } }
};
}
