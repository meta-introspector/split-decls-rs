// Generated macro for impl_211 (impl)
macro_rules! Depcrate_value_serimpl_211 {
() => {
// Module: crate::value::ser
// Provides: {"impl_211"}
// Dependencies: {}
impl serde :: ser :: SerializeTuple for SerializeVec { type Ok = Value ; type Error = Error ; fn serialize_element < T : ? Sized > (& mut self , value : & T) -> Result < () , Error > where T : Serialize , { serde :: ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Value , Error > { serde :: ser :: SerializeSeq :: end (self) } }
};
}
