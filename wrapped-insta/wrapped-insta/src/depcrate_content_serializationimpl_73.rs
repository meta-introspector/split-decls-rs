// Generated macro for impl_73 (impl)
macro_rules! Depcrate_content_serializationimpl_73 {
() => {
// Module: crate::content::serialization
// Provides: {"impl_73"}
// Dependencies: {}
impl < E > ser :: SerializeStruct for SerializeStruct < E > where E : ser :: Error , { type Ok = Content ; type Error = E ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , E > where T : Serialize + ? Sized , { let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . fields . push ((key , value)) ; Ok (()) } fn end (self) -> Result < Content , E > { Ok (Content :: Struct (self . name , self . fields)) } }
};
}
