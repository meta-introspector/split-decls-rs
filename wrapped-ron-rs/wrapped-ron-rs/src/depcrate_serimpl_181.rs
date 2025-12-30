// Generated macro for impl_181 (impl)
macro_rules! Depcrate_serimpl_181 {
() => {
// Module: crate::ser
// Provides: {"impl_181"}
// Dependencies: {}
impl < 'a , W : fmt :: Write > ser :: SerializeTuple for Compound < 'a , W > { type Error = Error ; type Ok = () ; fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { if let State :: First = self . state { self . state = State :: Rest ; } else { self . ser . output . write_char (',') ? ; if let Some ((ref config , ref pretty)) = self . ser . pretty { if pretty . indent <= config . depth_limit && self . ser . separate_tuple_members () { self . ser . output . write_str (& config . new_line) ? ; } else { self . ser . output . write_str (& config . separator) ? ; } } } if self . ser . separate_tuple_members () { self . ser . indent () ? ; } guard_recursion ! { self . ser => value . serialize (& mut * self . ser) ? } ; Ok (()) } fn end (self) -> Result < () > { if let State :: Rest = self . state { if let Some ((ref config , ref pretty)) = self . ser . pretty { if self . ser . separate_tuple_members () && pretty . indent <= config . depth_limit { self . ser . output . write_char (',') ? ; self . ser . output . write_str (& config . new_line) ? ; } } } if self . ser . separate_tuple_members () { self . ser . end_indent () ? ; } if ! self . newtype_variant { self . ser . output . write_char (')') ? ; } Ok (()) } }
};
}
