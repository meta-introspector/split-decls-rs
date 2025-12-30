// Generated macro for impl_146 (impl)
macro_rules! Depcrate_ext_seimpl_146 {
() => {
// Module: crate::ext::se
// Provides: {"impl_146"}
// Dependencies: {}
impl SerializeTuple for & mut ExtSerializer { type Ok = () ; type Error = Error ; # [inline] fn serialize_element < T : ? Sized > (& mut self , value : & T) -> Result < () , Error > where T : Serialize { if let Some (se) = & mut self . fields_se { value . serialize (se) } else { debug_assert ! (false) ; Err (Error :: Syntax (String :: new ())) } } # [inline (always)] fn end (self) -> Result < () , Error > { Ok (()) } }
};
}
