// Generated macro for impl_406 (impl)
macro_rules! Depcrate_value_serimpl_406 {
() => {
// Module: crate::value::ser
// Provides: {"impl_406"}
// Dependencies: {}
impl serde :: ser :: SerializeTuple for SerializeVec { type Ok = Value ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { serde :: ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Value > { serde :: ser :: SerializeSeq :: end (self) } }
};
}
