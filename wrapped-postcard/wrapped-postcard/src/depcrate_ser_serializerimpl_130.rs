// Generated macro for impl_130 (impl)
macro_rules! Depcrate_ser_serializerimpl_130 {
() => {
// Module: crate::ser::serializer
// Provides: {"impl_130"}
// Dependencies: {}
impl < F > ser :: SerializeTupleStruct for & mut Serializer < F > where F : Flavor , { type Ok = () ; type Error = Error ; # [inline] fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { value . serialize (& mut * * self) } # [inline] fn end (self) -> Result < () > { Ok (()) } }
};
}
