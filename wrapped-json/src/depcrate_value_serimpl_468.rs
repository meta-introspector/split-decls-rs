// Generated macro for impl_468 (impl)
macro_rules! Depcrate_value_serimpl_468 {
() => {
// Module: crate::value::ser
// Provides: {"impl_468"}
// Dependencies: {}
impl serde :: ser :: SerializeTupleStruct for SerializeVec { type Ok = Value ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { serde :: ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Value > { serde :: ser :: SerializeSeq :: end (self) } }
};
}
