// Generated macro for impl_135 (impl)
macro_rules! Depcrate_serimpl_135 {
() => {
// Module: crate::ser
// Provides: {"impl_135"}
// Dependencies: {}
impl < 'a , W > ser :: SerializeSeq for CollectionSerializer < 'a , W > where W : Write , { type Ok = () ; type Error = Error ; # [inline] fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { value . serialize (& mut * self . ser) } # [inline] fn end (self) -> Result < () > { self . end_inner () } }
};
}
