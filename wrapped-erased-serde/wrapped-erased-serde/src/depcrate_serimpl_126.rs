// Generated macro for impl_126 (impl)
macro_rules! Depcrate_serimpl_126 {
() => {
// Module: crate::ser
// Provides: {"impl_126"}
// Dependencies: {}
impl serde :: ser :: SerializeSeq for MakeSerializer < & mut dyn SerializeSeq > { type Ok = () ; type Error = ErrorImpl ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + serde :: Serialize , { self . 0 . erased_serialize_element (& value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . 0 . erased_end () ; Ok (()) } }
};
}
