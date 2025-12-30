// Generated macro for impl_545 (impl)
macro_rules! Depcrate_se_elementimpl_545 {
() => {
// Module: crate::se::element
// Provides: {"impl_545"}
// Dependencies: {}
impl < 'w , 'k , W : Write > SerializeStruct for Struct < 'w , 'k , W > { type Ok = WriteResult ; type Error = SeError ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { self . write_field (key , value) } fn end (mut self) -> Result < Self :: Ok , Self :: Error > { self . ser . ser . indent . decrease () ; if self . children . is_empty () { if self . ser . ser . expand_empty_elements { self . ser . ser . writer . write_str ("></") ? ; self . ser . ser . writer . write_str (self . ser . key . 0) ? ; self . ser . ser . writer . write_char ('>') ? ; } else { self . ser . ser . writer . write_str ("/>") ? ; } } else { self . ser . ser . writer . write_char ('>') ? ; self . ser . ser . writer . write_str (& self . children) ? ; if self . write_indent { self . ser . ser . indent . write_indent (& mut self . ser . ser . writer) ? ; } self . ser . ser . writer . write_str ("</") ? ; self . ser . ser . writer . write_str (self . ser . key . 0) ? ; self . ser . ser . writer . write_char ('>') ? ; } Ok (WriteResult :: Element) } }
};
}
