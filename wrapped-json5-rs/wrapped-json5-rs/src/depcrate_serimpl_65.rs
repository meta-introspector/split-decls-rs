// Generated macro for impl_65 (impl)
macro_rules! Depcrate_serimpl_65 {
() => {
// Module: crate::ser
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'a > ser :: SerializeStruct for & 'a mut Serializer { type Ok = () ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () > where T : ? Sized + Serialize , { ser :: SerializeMap :: serialize_key (self , key) ? ; ser :: SerializeMap :: serialize_value (self , value) } fn end (self) -> Result < () > { ser :: SerializeMap :: end (self) } }
};
}
