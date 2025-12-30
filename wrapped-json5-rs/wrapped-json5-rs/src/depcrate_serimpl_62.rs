// Generated macro for impl_62 (impl)
macro_rules! Depcrate_serimpl_62 {
() => {
// Module: crate::ser
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a > ser :: SerializeTupleStruct for & 'a mut Serializer { type Ok = () ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < () > { ser :: SerializeSeq :: end (self) } }
};
}
