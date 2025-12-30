// Generated macro for impl_549 (impl)
macro_rules! Depcrate_se_elementimpl_549 {
() => {
// Module: crate::se::element
// Provides: {"impl_549"}
// Dependencies: {}
impl < 'w , 'k , W : Write > SerializeMap for Map < 'w , 'k , W > { type Ok = WriteResult ; type Error = SeError ; fn serialize_key < T > (& mut self , key : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { if self . key . take () . is_some () { return Err (SeError :: Custom ("calling `serialize_key` twice without `serialize_value`" . to_string () ,)) ; } self . key = Some (self . make_key (key) ?) ; Ok (()) } fn serialize_value < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { if let Some (key) = self . key . take () { return self . ser . write_field (& key , value) ; } Err (SeError :: Custom ("calling `serialize_value` without call of `serialize_key`" . to_string () ,)) } fn serialize_entry < K , V > (& mut self , key : & K , value : & V) -> Result < () , Self :: Error > where K : ? Sized + Serialize , V : ? Sized + Serialize , { let key = self . make_key (key) ? ; self . ser . write_field (& key , value) } fn end (mut self) -> Result < Self :: Ok , Self :: Error > { if let Some (key) = self . key . take () { return Err (SeError :: Custom (format ! ("calling `end` without call of `serialize_value` for key `{key}`"))) ; } SerializeStruct :: end (self . ser) } }
};
}
