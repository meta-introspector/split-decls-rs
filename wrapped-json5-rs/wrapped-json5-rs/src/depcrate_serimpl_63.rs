// Generated macro for impl_63 (impl)
macro_rules! Depcrate_serimpl_63 {
() => {
// Module: crate::ser
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a > ser :: SerializeTupleVariant for & 'a mut Serializer { type Ok = () ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < () > { self . output += "]}" ; Ok (()) } }
};
}
