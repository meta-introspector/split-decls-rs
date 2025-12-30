// Generated macro for impl_60 (impl)
macro_rules! Depcrate_serimpl_60 {
() => {
// Module: crate::ser
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a > ser :: SerializeSeq for & 'a mut Serializer { type Ok = () ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { if ! self . output . ends_with ('[') { self . output += "," ; } value . serialize (& mut * * self) } fn end (self) -> Result < () > { self . output += "]" ; Ok (()) } }
};
}
