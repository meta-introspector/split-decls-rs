// Generated macro for impl_128 (impl)
macro_rules! Depcrate_serimpl_128 {
() => {
// Module: crate::ser
// Provides: {"impl_128"}
// Dependencies: {}
impl < 'a , W > ser :: SerializeTupleVariant for & 'a mut Serializer < W > where W : Write , { type Ok = () ; type Error = Error ; # [inline] fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { value . serialize (& mut * * self) } # [inline] fn end (self) -> Result < () > { Ok (()) } }
};
}
