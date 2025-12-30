// Generated macro for impl_63 (impl)
macro_rules! Depcrate_content_serializationimpl_63 {
() => {
// Module: crate::content::serialization
// Provides: {"impl_63"}
// Dependencies: {}
impl < E > ser :: SerializeSeq for SerializeSeq < E > where E : ser :: Error , { type Ok = Content ; type Error = E ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , E > where T : Serialize + ? Sized , { let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . elements . push (value) ; Ok (()) } fn end (self) -> Result < Content , E > { Ok (Content :: Seq (self . elements)) } }
};
}
