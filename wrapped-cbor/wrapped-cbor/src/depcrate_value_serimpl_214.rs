// Generated macro for impl_214 (impl)
macro_rules! Depcrate_value_serimpl_214 {
() => {
// Module: crate::value::ser
// Provides: {"impl_214"}
// Dependencies: {}
impl serde :: ser :: SerializeMap for SerializeMap { type Ok = Value ; type Error = Error ; fn serialize_key < T : ? Sized > (& mut self , key : & T) -> Result < () , Error > where T : Serialize , { self . next_key = Some (to_value (& key) ?) ; Ok (()) } fn serialize_value < T : ? Sized > (& mut self , value : & T) -> Result < () , Error > where T : Serialize , { let key = self . next_key . take () ; let key = key . expect ("serialize_value called before serialize_key") ; self . map . insert (key , to_value (& value) ?) ; Ok (()) } fn end (self) -> Result < Value , Error > { Ok (Value :: Map (self . map)) } }
};
}
