// Generated macro for impl_466 (impl)
macro_rules! Depcrate_value_serimpl_466 {
() => {
// Module: crate::value::ser
// Provides: {"impl_466"}
// Dependencies: {}
impl serde :: ser :: SerializeSeq for SerializeVec { type Ok = Value ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { self . vec . push (tri ! (to_value (value))) ; Ok (()) } fn end (self) -> Result < Value > { Ok (Value :: Array (self . vec)) } }
};
}
