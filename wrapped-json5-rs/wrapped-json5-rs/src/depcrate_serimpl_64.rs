// Generated macro for impl_64 (impl)
macro_rules! Depcrate_serimpl_64 {
() => {
// Module: crate::ser
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a > ser :: SerializeMap for & 'a mut Serializer { type Ok = () ; type Error = Error ; fn serialize_key < T > (& mut self , key : & T) -> Result < () > where T : ? Sized + Serialize , { if ! self . output . ends_with ('{') { self . output += "," ; } key . serialize (& mut * * self) } fn serialize_value < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { self . output += ":" ; value . serialize (& mut * * self) } fn end (self) -> Result < () > { self . output += "}" ; Ok (()) } }
};
}
