// Generated macro for impl_126 (impl)
macro_rules! Depcrate_ser_serializerimpl_126 {
() => {
// Module: crate::ser::serializer
// Provides: {"impl_126"}
// Dependencies: {}
impl < F > ser :: SerializeTupleVariant for & mut Serializer < F > where F : Flavor , { type Ok = () ; type Error = Error ; # [inline] fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { value . serialize (& mut * * self) } # [inline] fn end (self) -> Result < () > { Ok (()) } }
};
}
