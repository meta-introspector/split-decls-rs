// Generated macro for impl_212 (impl)
macro_rules! Depcrate_value_serimpl_212 {
() => {
// Module: crate::value::ser
// Provides: {"impl_212"}
// Dependencies: {}
impl serde :: ser :: SerializeTupleStruct for SerializeVec { type Ok = Value ; type Error = Error ; fn serialize_field < T : ? Sized > (& mut self , value : & T) -> Result < () , Error > where T : Serialize , { serde :: ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Value , Error > { serde :: ser :: SerializeSeq :: end (self) } }
};
}
