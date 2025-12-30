// Generated macro for impl_186 (impl)
macro_rules! Depcrate_serimpl_186 {
() => {
// Module: crate::ser
// Provides: {"impl_186"}
// Dependencies: {}
impl < 'a , W : fmt :: Write > ser :: SerializeStructVariant for Compound < 'a , W > { type Error = Error ; type Ok = () ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () > where T : ? Sized + Serialize , { ser :: SerializeStruct :: serialize_field (self , key , value) } fn end (self) -> Result < () > { ser :: SerializeStruct :: end (self) } }
};
}
