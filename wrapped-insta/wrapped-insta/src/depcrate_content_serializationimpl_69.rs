// Generated macro for impl_69 (impl)
macro_rules! Depcrate_content_serializationimpl_69 {
() => {
// Module: crate::content::serialization
// Provides: {"impl_69"}
// Dependencies: {}
impl < E > ser :: SerializeTupleVariant for SerializeTupleVariant < E > where E : ser :: Error , { type Ok = Content ; type Error = E ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , E > where T : Serialize + ? Sized , { let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . fields . push (value) ; Ok (()) } fn end (self) -> Result < Content , E > { Ok (Content :: TupleVariant (self . name , self . variant_index , self . variant , self . fields ,)) } }
};
}
