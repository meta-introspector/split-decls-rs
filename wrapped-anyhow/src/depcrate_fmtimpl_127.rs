// Generated macro for impl_127 (impl)
macro_rules! Depcrate_fmtimpl_127 {
() => {
// Module: crate::fmt
// Provides: {"impl_127"}
// Dependencies: {}
impl < T > Write for Indented < '_ , T > where T : Write , { fn write_str (& mut self , s : & str) -> fmt :: Result { for (i , line) in s . split ('\n') . enumerate () { if ! self . started { self . started = true ; match self . number { Some (number) => write ! (self . inner , "{: >5}: " , number) ? , None => self . inner . write_str ("    ") ? , } } else if i > 0 { self . inner . write_char ('\n') ? ; if self . number . is_some () { self . inner . write_str ("       ") ? ; } else { self . inner . write_str ("    ") ? ; } } self . inner . write_str (line) ? ; } Ok (()) } }
};
}
