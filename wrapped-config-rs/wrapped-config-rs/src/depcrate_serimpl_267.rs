// Generated macro for impl_267 (impl)
macro_rules! Depcrate_serimpl_267 {
() => {
// Module: crate::ser
// Provides: {"impl_267"}
// Dependencies: {}
impl ser :: SerializeMap for & mut ConfigSerializer { type Ok = () ; type Error = ConfigError ; fn serialize_key < T > (& mut self , key : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { let key_serializer = StringKeySerializer ; let key = key . serialize (key_serializer) ? ; self . push_key (& key) ; Ok (()) } fn serialize_value < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { value . serialize (& mut * * self) ? ; self . pop_key () ; Ok (()) } fn end (self) -> Result < Self :: Ok > { Ok (()) } }
};
}
