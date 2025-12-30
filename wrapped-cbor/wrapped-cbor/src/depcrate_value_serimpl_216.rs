// Generated macro for impl_216 (impl)
macro_rules! Depcrate_value_serimpl_216 {
() => {
// Module: crate::value::ser
// Provides: {"impl_216"}
// Dependencies: {}
impl serde :: ser :: SerializeStructVariant for SerializeStructVariant { type Ok = Value ; type Error = Error ; fn serialize_field < T : ? Sized > (& mut self , key : & 'static str , value : & T) -> Result < () , Error > where T : Serialize , { self . map . insert (Value :: from (String :: from (key)) , to_value (& value) ?) ; Ok (()) } fn end (self) -> Result < Value , Error > { let mut object = BTreeMap :: new () ; object . insert (Value :: from (self . name) , Value :: Map (self . map)) ; Ok (Value :: Map (object)) } }
};
}
