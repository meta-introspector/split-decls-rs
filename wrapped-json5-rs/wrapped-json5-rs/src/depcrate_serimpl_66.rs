// Generated macro for impl_66 (impl)
macro_rules! Depcrate_serimpl_66 {
() => {
// Module: crate::ser
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a > ser :: SerializeStructVariant for & 'a mut Serializer { type Ok = () ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () > where T : ? Sized + Serialize , { ser :: SerializeStruct :: serialize_field (self , key , value) } fn end (self) -> Result < () > { self . output += "}}" ; Ok (()) } }
};
}
