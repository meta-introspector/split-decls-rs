// Generated macro for impl_579 (impl)
macro_rules! Depcrate_se_simple_typeimpl_579 {
() => {
// Module: crate::se::simple_type
// Provides: {"impl_579"}
// Dependencies: {}
impl < W : Write > SerializeTupleStruct for SimpleSeq < W > { type Ok = W ; type Error = SeError ; # [inline] fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { SerializeSeq :: serialize_element (self , value) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { SerializeSeq :: end (self) } }
};
}
