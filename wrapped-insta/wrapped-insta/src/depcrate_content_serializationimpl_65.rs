// Generated macro for impl_65 (impl)
macro_rules! Depcrate_content_serializationimpl_65 {
() => {
// Module: crate::content::serialization
// Provides: {"impl_65"}
// Dependencies: {}
impl < E > ser :: SerializeTuple for SerializeTuple < E > where E : ser :: Error , { type Ok = Content ; type Error = E ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , E > where T : Serialize + ? Sized , { let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . elements . push (value) ; Ok (()) } fn end (self) -> Result < Content , E > { Ok (Content :: Tuple (self . elements)) } }
};
}
