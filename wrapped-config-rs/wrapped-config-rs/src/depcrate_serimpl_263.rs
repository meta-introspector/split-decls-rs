// Generated macro for impl_263 (impl)
macro_rules! Depcrate_serimpl_263 {
() => {
// Module: crate::ser
// Provides: {"impl_263"}
// Dependencies: {}
impl ser :: SerializeSeq for SeqSerializer < '_ > { type Ok = () ; type Error = ConfigError ; fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { value . serialize (& mut * (self . 0)) ? ; match self . 0 . keys . last_mut () { Some (SerKey :: Seq (i)) => * i += 1 , _ => { return Err (ConfigError :: Message ("config-rs internal error (ser._element but last not Seq!" . to_owned () ,)) } } ; Ok (()) } fn end (self) -> Result < Self :: Ok > { self . end () ; Ok (()) } }
};
}
