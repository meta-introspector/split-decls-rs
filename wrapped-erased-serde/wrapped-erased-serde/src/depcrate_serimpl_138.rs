// Generated macro for impl_138 (impl)
macro_rules! Depcrate_serimpl_138 {
() => {
// Module: crate::ser
// Provides: {"impl_138"}
// Dependencies: {}
impl serde :: ser :: SerializeMap for MakeSerializer < & mut dyn SerializeMap > { type Ok = () ; type Error = ErrorImpl ; fn serialize_key < T > (& mut self , key : & T) -> Result < () , Self :: Error > where T : ? Sized + serde :: Serialize , { self . 0 . erased_serialize_key (& key) } fn serialize_value < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + serde :: Serialize , { self . 0 . erased_serialize_value (& value) } fn serialize_entry < K , V > (& mut self , key : & K , value : & V) -> Result < () , Self :: Error > where K : ? Sized + serde :: Serialize , V : ? Sized + serde :: Serialize , { self . 0 . erased_serialize_entry (& key , & value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . 0 . erased_end () ; Ok (()) } }
};
}
