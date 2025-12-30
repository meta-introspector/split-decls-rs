// Generated macro for impl_202 (impl)
macro_rules! Depcrate_serimpl_202 {
() => {
// Module: crate::ser
// Provides: {"impl_202"}
// Dependencies: {}
impl < 'a , W , F > ser :: SerializeTupleStruct for Compound < 'a , W , F > where W : io :: Write , F : Formatter , { type Ok = () ; type Error = Error ; # [inline] fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { ser :: SerializeSeq :: serialize_element (self , value) } # [inline] fn end (self) -> Result < () > { ser :: SerializeSeq :: end (self) } }
};
}
