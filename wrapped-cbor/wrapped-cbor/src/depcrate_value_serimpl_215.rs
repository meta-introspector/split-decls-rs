// Generated macro for impl_215 (impl)
macro_rules! Depcrate_value_serimpl_215 {
() => {
// Module: crate::value::ser
// Provides: {"impl_215"}
// Dependencies: {}
impl serde :: ser :: SerializeStruct for SerializeMap { type Ok = Value ; type Error = Error ; fn serialize_field < T : ? Sized > (& mut self , key : & 'static str , value : & T) -> Result < () , Error > where T : Serialize , { serde :: ser :: SerializeMap :: serialize_key (self , key) ? ; serde :: ser :: SerializeMap :: serialize_value (self , value) } fn end (self) -> Result < Value , Error > { serde :: ser :: SerializeMap :: end (self) } }
};
}
