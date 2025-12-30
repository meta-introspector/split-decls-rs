// Generated macro for impl_184 (impl)
macro_rules! Depcrate_serimpl_184 {
() => {
// Module: crate::ser
// Provides: {"impl_184"}
// Dependencies: {}
impl < 'a , W : fmt :: Write > ser :: SerializeMap for Compound < 'a , W > { type Error = Error ; type Ok = () ; fn serialize_key < T > (& mut self , key : & T) -> Result < () > where T : ? Sized + Serialize , { if let State :: First = self . state { self . state = State :: Rest ; } else { self . ser . output . write_char (',') ? ; if let Some ((ref config , ref pretty)) = self . ser . pretty { if pretty . indent <= config . depth_limit && ! config . compact_maps { self . ser . output . write_str (& config . new_line) ? ; } else { self . ser . output . write_str (& config . separator) ? ; } } } if ! self . ser . compact_maps () { self . ser . indent () ? ; } guard_recursion ! { self . ser => key . serialize (& mut * self . ser) } } fn serialize_value < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { self . ser . output . write_char (':') ? ; if let Some ((ref config , _)) = self . ser . pretty { self . ser . output . write_str (& config . separator) ? ; } guard_recursion ! { self . ser => value . serialize (& mut * self . ser) ? } ; Ok (()) } fn end (self) -> Result < () > { if let State :: Rest = self . state { if let Some ((ref config , ref pretty)) = self . ser . pretty { if pretty . indent <= config . depth_limit && ! config . compact_maps { self . ser . output . write_char (',') ? ; self . ser . output . write_str (& config . new_line) ? ; } } } if ! self . ser . compact_maps () { self . ser . end_indent () ? ; } self . ser . output . write_char ('}') ? ; Ok (()) } }
};
}
