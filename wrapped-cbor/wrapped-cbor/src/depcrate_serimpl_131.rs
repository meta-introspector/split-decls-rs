// Generated macro for impl_131 (impl)
macro_rules! Depcrate_serimpl_131 {
() => {
// Module: crate::ser
// Provides: {"impl_131"}
// Dependencies: {}
impl < 'a , W > ser :: SerializeStruct for StructSerializer < 'a , W > where W : Write , { type Ok = () ; type Error = Error ; # [inline] fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { self . serialize_field_inner (key , value) } # [inline] fn skip_field (& mut self , key : & 'static str) -> Result < () > { self . skip_field_inner (key) } # [inline] fn end (self) -> Result < () > { self . end_inner () } }
};
}
