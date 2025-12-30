// Generated macro for impl_407 (impl)
macro_rules! Depcrate_value_serimpl_407 {
() => {
// Module: crate::value::ser
// Provides: {"impl_407"}
// Dependencies: {}
impl serde :: ser :: SerializeTupleStruct for SerializeVec { type Ok = Value ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { serde :: ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Value > { serde :: ser :: SerializeSeq :: end (self) } }
};
}
