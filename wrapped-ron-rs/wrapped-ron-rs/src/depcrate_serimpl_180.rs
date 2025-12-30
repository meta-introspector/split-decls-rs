// Generated macro for impl_180 (impl)
macro_rules! Depcrate_serimpl_180 {
() => {
// Module: crate::ser
// Provides: {"impl_180"}
// Dependencies: {}
impl < 'a , W : fmt :: Write > ser :: SerializeSeq for Compound < 'a , W > { type Error = Error ; type Ok = () ; fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { if let State :: First = self . state { self . state = State :: Rest ; } else { self . ser . output . write_char (',') ? ; if let Some ((ref config , ref mut pretty)) = self . ser . pretty { if pretty . indent <= config . depth_limit && ! config . compact_arrays { self . ser . output . write_str (& config . new_line) ? ; } else { self . ser . output . write_str (& config . separator) ? ; } } } if ! self . ser . compact_arrays () { self . ser . indent () ? ; } if let Some ((ref mut config , ref mut pretty)) = self . ser . pretty { if pretty . indent <= config . depth_limit && config . enumerate_arrays { write ! (self . ser . output , "/*[{}]*/ " , self . sequence_index) ? ; self . sequence_index += 1 ; } } guard_recursion ! { self . ser => value . serialize (& mut * self . ser) ? } ; Ok (()) } fn end (self) -> Result < () > { if let State :: Rest = self . state { if let Some ((ref config , ref mut pretty)) = self . ser . pretty { if pretty . indent <= config . depth_limit && ! config . compact_arrays { self . ser . output . write_char (',') ? ; self . ser . output . write_str (& config . new_line) ? ; } } } if ! self . ser . compact_arrays () { self . ser . end_indent () ? ; } self . ser . output . write_char (']') ? ; Ok (()) } }
};
}
