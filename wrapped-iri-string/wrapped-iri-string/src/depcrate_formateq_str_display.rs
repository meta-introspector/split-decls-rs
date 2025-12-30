// Generated macro for eq_str_display (function)
macro_rules! Depcrate_formateq_str_display {
() => {
// Module: crate::format
// Provides: {"eq_str_display"}
// Dependencies: {}
# [doc = " Returns true if the two equals after they are converted to strings."] pub (crate) fn eq_str_display < T > (s : & str , d : & T) -> bool where T : ? Sized + fmt :: Display , { # [doc = " Dummy writer to compare the formatted object to the given string."] struct CmpWriter < 'a > (& 'a str) ; impl fmt :: Write for CmpWriter < '_ > { fn write_str (& mut self , s : & str) -> fmt :: Result { if self . 0 . len () < s . len () { return Err (fmt :: Error) ; } let (prefix , rest) = self . 0 . split_at (s . len ()) ; self . 0 = rest ; if prefix == s { Ok (()) } else { Err (fmt :: Error) } } } let mut writer = CmpWriter (s) ; let succeeded = write ! (writer , "{}" , d) . is_ok () ; succeeded && writer . 0 . is_empty () }
};
}
