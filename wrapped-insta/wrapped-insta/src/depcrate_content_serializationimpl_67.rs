// Generated macro for impl_67 (impl)
macro_rules! Depcrate_content_serializationimpl_67 {
() => {
// Module: crate::content::serialization
// Provides: {"impl_67"}
// Dependencies: {}
impl < E > ser :: SerializeTupleStruct for SerializeTupleStruct < E > where E : ser :: Error , { type Ok = Content ; type Error = E ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , E > where T : Serialize + ? Sized , { let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . fields . push (value) ; Ok (()) } fn end (self) -> Result < Content , E > { Ok (Content :: TupleStruct (self . name , self . fields)) } }
};
}
