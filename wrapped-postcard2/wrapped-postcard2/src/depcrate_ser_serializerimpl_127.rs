// Generated macro for impl_127 (impl)
macro_rules! Depcrate_ser_serializerimpl_127 {
() => {
// Module: crate::ser::serializer
// Provides: {"impl_127"}
// Dependencies: {}
impl < F > ser :: SerializeMap for & mut Serializer < F > where F : Flavor , { type Ok = () ; type Error = Error ; # [inline] fn serialize_key < T > (& mut self , key : & T) -> Result < () > where T : ? Sized + Serialize , { key . serialize (& mut * * self) } # [inline] fn serialize_value < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { value . serialize (& mut * * self) } # [inline] fn end (self) -> Result < () > { Ok (()) } }
};
}
