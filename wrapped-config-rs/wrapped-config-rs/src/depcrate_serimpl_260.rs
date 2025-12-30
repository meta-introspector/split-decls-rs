// Generated macro for impl_260 (impl)
macro_rules! Depcrate_serimpl_260 {
() => {
// Module: crate::ser
// Provides: {"impl_260"}
// Dependencies: {}
impl ConfigSerializer { fn serialize_primitive < T > (& mut self , value : T) -> Result < () > where T : Into < Value > + Display , { let key = self . make_full_key () ? ; self . output . set (& key , value . into ()) ? ; Ok (()) } fn make_full_key (& self) -> Result < String > { let mut keys = self . keys . iter () ; let mut whole = match keys . next () { Some (SerKey :: Named (s)) => s . clone () , _ => return Err (ConfigError :: Message ("top level is not a struct" . to_owned ())) , } ; for k in keys { match k { SerKey :: Named (s) => write ! (whole , ".{s}") , SerKey :: Seq (i) => write ! (whole , "[{i}]") , } . expect ("write! to a string failed") ; } Ok (whole) } fn push_key (& mut self , key : & str) { self . keys . push (SerKey :: Named (key . to_owned ())) ; } fn pop_key (& mut self) { self . keys . pop () ; } }
};
}
