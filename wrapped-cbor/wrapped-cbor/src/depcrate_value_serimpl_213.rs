// Generated macro for impl_213 (impl)
macro_rules! Depcrate_value_serimpl_213 {
() => {
// Module: crate::value::ser
// Provides: {"impl_213"}
// Dependencies: {}
impl serde :: ser :: SerializeTupleVariant for SerializeTupleVariant { type Ok = Value ; type Error = Error ; fn serialize_field < T : ? Sized > (& mut self , value : & T) -> Result < () , Error > where T : Serialize , { self . vec . push (to_value (& value) ?) ; Ok (()) } fn end (self) -> Result < Value , Error > { let mut object = BTreeMap :: new () ; object . insert (Value :: from (self . name) , Value :: Array (self . vec)) ; Ok (Value :: Map (object)) } }
};
}
