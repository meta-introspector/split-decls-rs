// Generated macro for impl_132 (impl)
macro_rules! Depcrate_serimpl_132 {
() => {
// Module: crate::ser
// Provides: {"impl_132"}
// Dependencies: {}
impl serde :: ser :: SerializeTupleStruct for MakeSerializer < & mut dyn SerializeTupleStruct > { type Ok = () ; type Error = ErrorImpl ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + serde :: Serialize , { self . 0 . erased_serialize_field (& value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . 0 . erased_end () ; Ok (()) } }
};
}
