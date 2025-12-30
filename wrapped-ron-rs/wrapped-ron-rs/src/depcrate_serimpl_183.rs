// Generated macro for impl_183 (impl)
macro_rules! Depcrate_serimpl_183 {
() => {
// Module: crate::ser
// Provides: {"impl_183"}
// Dependencies: {}
impl < 'a , W : fmt :: Write > ser :: SerializeTupleVariant for Compound < 'a , W > { type Error = Error ; type Ok = () ; fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { ser :: SerializeTuple :: serialize_element (self , value) } fn end (self) -> Result < () > { ser :: SerializeTuple :: end (self) } }
};
}
