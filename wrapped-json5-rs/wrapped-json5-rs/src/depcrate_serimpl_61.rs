// Generated macro for impl_61 (impl)
macro_rules! Depcrate_serimpl_61 {
() => {
// Module: crate::ser
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'a > ser :: SerializeTuple for & 'a mut Serializer { type Ok = () ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < () > { ser :: SerializeSeq :: end (self) } }
};
}
