// Generated macro for impl_546 (impl)
macro_rules! Depcrate_se_elementimpl_546 {
() => {
// Module: crate::se::element
// Provides: {"impl_546"}
// Dependencies: {}
impl < 'w , 'k , W : Write > SerializeStructVariant for Struct < 'w , 'k , W > { type Ok = WriteResult ; type Error = SeError ; # [inline] fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { SerializeStruct :: serialize_field (self , key , value) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { SerializeStruct :: end (self) } }
};
}
