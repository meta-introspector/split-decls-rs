// Generated macro for impl_476 (impl)
macro_rules! Depcrate_value_serimpl_476 {
() => {
// Module: crate::value::ser
// Provides: {"impl_476"}
// Dependencies: {}
impl serde :: ser :: SerializeStructVariant for SerializeStructVariant { type Ok = Value ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () > where T : ? Sized + Serialize , { self . map . insert (String :: from (key) , tri ! (to_value (value))) ; Ok (()) } fn end (self) -> Result < Value > { let mut object = Map :: new () ; object . insert (self . name , Value :: Object (self . map)) ; Ok (Value :: Object (object)) } }
};
}
