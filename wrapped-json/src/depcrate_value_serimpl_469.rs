// Generated macro for impl_469 (impl)
macro_rules! Depcrate_value_serimpl_469 {
() => {
// Module: crate::value::ser
// Provides: {"impl_469"}
// Dependencies: {}
impl serde :: ser :: SerializeTupleVariant for SerializeTupleVariant { type Ok = Value ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { self . vec . push (tri ! (to_value (value))) ; Ok (()) } fn end (self) -> Result < Value > { let mut object = Map :: new () ; object . insert (self . name , Value :: Array (self . vec)) ; Ok (Value :: Object (object)) } }
};
}
