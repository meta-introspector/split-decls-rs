// Generated macro for impl_134 (impl)
macro_rules! Depcrate_ser_serializerimpl_134 {
() => {
// Module: crate::ser::serializer
// Provides: {"impl_134"}
// Dependencies: {}
impl < F > ser :: SerializeStructVariant for & mut Serializer < F > where F : Flavor , { type Ok = () ; type Error = Error ; # [inline] fn serialize_field < T > (& mut self , _key : & 'static str , value : & T) -> Result < () > where T : ? Sized + Serialize , { value . serialize (& mut * * self) } # [inline] fn end (self) -> Result < () > { Ok (()) } }
};
}
