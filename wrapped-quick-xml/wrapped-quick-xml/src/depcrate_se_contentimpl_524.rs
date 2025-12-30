// Generated macro for impl_524 (impl)
macro_rules! Depcrate_se_contentimpl_524 {
() => {
// Module: crate::se::content
// Provides: {"impl_524"}
// Dependencies: {}
impl < 'w , 'i , W : Write > SerializeTupleStruct for Seq < 'w , 'i , W > { type Ok = WriteResult ; type Error = SeError ; # [inline] fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { SerializeSeq :: serialize_element (self , value) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { SerializeSeq :: end (self) } }
};
}
