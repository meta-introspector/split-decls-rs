// Generated macro for impl_71 (impl)
macro_rules! Depcrate_content_serializationimpl_71 {
() => {
// Module: crate::content::serialization
// Provides: {"impl_71"}
// Dependencies: {}
impl < E > ser :: SerializeMap for SerializeMap < E > where E : ser :: Error , { type Ok = Content ; type Error = E ; fn serialize_key < T > (& mut self , key : & T) -> Result < () , E > where T : Serialize + ? Sized , { let key = key . serialize (ContentSerializer :: < E > :: new ()) ? ; self . key = Some (key) ; Ok (()) } fn serialize_value < T > (& mut self , value : & T) -> Result < () , E > where T : Serialize + ? Sized , { let key = self . key . take () . expect ("serialize_value called before serialize_key") ; let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . entries . push ((key , value)) ; Ok (()) } fn end (self) -> Result < Content , E > { Ok (Content :: Map (self . entries)) } fn serialize_entry < K , V > (& mut self , key : & K , value : & V) -> Result < () , E > where K : Serialize + ? Sized , V : Serialize + ? Sized , { let key = key . serialize (ContentSerializer :: < E > :: new ()) ? ; let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . entries . push ((key , value)) ; Ok (()) } }
};
}
