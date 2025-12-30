// Generated macro for impl_75 (impl)
macro_rules! Depcrate_content_serializationimpl_75 {
() => {
// Module: crate::content::serialization
// Provides: {"impl_75"}
// Dependencies: {}
impl < E > ser :: SerializeStructVariant for SerializeStructVariant < E > where E : ser :: Error , { type Ok = Content ; type Error = E ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , E > where T : Serialize + ? Sized , { let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . fields . push ((key , value)) ; Ok (()) } fn end (self) -> Result < Content , E > { Ok (Content :: StructVariant (self . name , self . variant_index , self . variant , self . fields ,)) } }
};
}
