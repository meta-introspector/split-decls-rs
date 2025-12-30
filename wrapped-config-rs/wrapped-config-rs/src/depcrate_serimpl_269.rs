// Generated macro for impl_269 (impl)
macro_rules! Depcrate_serimpl_269 {
() => {
// Module: crate::ser
// Provides: {"impl_269"}
// Dependencies: {}
impl ser :: SerializeStructVariant for & mut ConfigSerializer { type Ok = () ; type Error = ConfigError ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { self . push_key (key) ; value . serialize (& mut * * self) ? ; self . pop_key () ; Ok (()) } fn end (self) -> Result < Self :: Ok > { self . pop_key () ; Ok (()) } }
};
}
