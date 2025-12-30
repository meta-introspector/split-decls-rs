// Generated macro for impl_123 (impl)
macro_rules! Depcrate_ser_serializerimpl_123 {
() => {
// Module: crate::ser::serializer
// Provides: {"impl_123"}
// Dependencies: {}
impl < F > ser :: SerializeSeq for & mut Serializer < F > where F : Flavor , { type Ok = () ; type Error = Error ; # [inline] fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { value . serialize (& mut * * self) } # [inline] fn end (self) -> Result < () > { Ok (()) } }
};
}
