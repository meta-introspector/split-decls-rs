// Generated macro for impl_182 (impl)
macro_rules! Depcrate_serimpl_182 {
() => {
// Module: crate::ser
// Provides: {"impl_182"}
// Dependencies: {}
impl < 'a , W : fmt :: Write > ser :: SerializeTupleStruct for Compound < 'a , W > { type Error = Error ; type Ok = () ; fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { ser :: SerializeTuple :: serialize_element (self , value) } fn end (self) -> Result < () > { ser :: SerializeTuple :: end (self) } }
};
}
