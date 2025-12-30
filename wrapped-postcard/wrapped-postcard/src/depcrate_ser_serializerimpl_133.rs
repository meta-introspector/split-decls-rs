// Generated macro for impl_133 (impl)
macro_rules! Depcrate_ser_serializerimpl_133 {
() => {
// Module: crate::ser::serializer
// Provides: {"impl_133"}
// Dependencies: {}
impl < F > ser :: SerializeStruct for & mut Serializer < F > where F : Flavor , { type Ok = () ; type Error = Error ; # [inline] fn serialize_field < T > (& mut self , _key : & 'static str , value : & T) -> Result < () > where T : ? Sized + Serialize , { value . serialize (& mut * * self) } # [inline] fn end (self) -> Result < () > { Ok (()) } }
};
}
