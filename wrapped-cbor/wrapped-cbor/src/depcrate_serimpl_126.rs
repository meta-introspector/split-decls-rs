// Generated macro for impl_126 (impl)
macro_rules! Depcrate_serimpl_126 {
() => {
// Module: crate::ser
// Provides: {"impl_126"}
// Dependencies: {}
impl < 'a , W > ser :: SerializeTuple for & 'a mut Serializer < W > where W : Write , { type Ok = () ; type Error = Error ; # [inline] fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { value . serialize (& mut * * self) } # [inline] fn end (self) -> Result < () > { Ok (()) } }
};
}
