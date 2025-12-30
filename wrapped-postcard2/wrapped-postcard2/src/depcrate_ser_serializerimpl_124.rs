// Generated macro for impl_124 (impl)
macro_rules! Depcrate_ser_serializerimpl_124 {
() => {
// Module: crate::ser::serializer
// Provides: {"impl_124"}
// Dependencies: {}
impl < F > ser :: SerializeTuple for & mut Serializer < F > where F : Flavor , { type Ok = () ; type Error = Error ; # [inline] fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { value . serialize (& mut * * self) } # [inline] fn end (self) -> Result < () > { Ok (()) } }
};
}
