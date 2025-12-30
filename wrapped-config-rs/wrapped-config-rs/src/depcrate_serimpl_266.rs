// Generated macro for impl_266 (impl)
macro_rules! Depcrate_serimpl_266 {
() => {
// Module: crate::ser
// Provides: {"impl_266"}
// Dependencies: {}
impl ser :: SerializeTupleVariant for SeqSerializer < '_ > { type Ok = () ; type Error = ConfigError ; fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Self :: Ok > { let inner = self . end () ; inner . pop_key () ; Ok (()) } }
};
}
