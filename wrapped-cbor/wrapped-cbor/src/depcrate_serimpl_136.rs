// Generated macro for impl_136 (impl)
macro_rules! Depcrate_serimpl_136 {
() => {
// Module: crate::ser
// Provides: {"impl_136"}
// Dependencies: {}
impl < 'a , W > ser :: SerializeMap for CollectionSerializer < 'a , W > where W : Write , { type Ok = () ; type Error = Error ; # [inline] fn serialize_key < T > (& mut self , key : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { key . serialize (& mut * self . ser) } # [inline] fn serialize_value < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { value . serialize (& mut * self . ser) } # [inline] fn end (self) -> Result < () > { self . end_inner () } }
};
}
