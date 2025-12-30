// Generated macro for impl_210 (impl)
macro_rules! Depcrate_value_serimpl_210 {
() => {
// Module: crate::value::ser
// Provides: {"impl_210"}
// Dependencies: {}
impl serde :: ser :: SerializeSeq for SerializeVec { type Ok = Value ; type Error = Error ; fn serialize_element < T : ? Sized > (& mut self , value : & T) -> Result < () , Error > where T : Serialize , { self . vec . push (to_value (& value) ?) ; Ok (()) } fn end (self) -> Result < Value , Error > { Ok (Value :: Array (self . vec)) } }
};
}
