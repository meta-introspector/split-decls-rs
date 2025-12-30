// Generated macro for impl_800 (impl)
macro_rules! Depcrate_dotimpl_800 {
() => {
// Module: crate::dot
// Provides: {"impl_800"}
// Dependencies: {}
impl < W > fmt :: Write for Escaper < W > where W : fmt :: Write , { fn write_str (& mut self , s : & str) -> fmt :: Result { for c in s . chars () { self . write_char (c) ? ; } Ok (()) } fn write_char (& mut self , c : char) -> fmt :: Result { match c { '"' | '\\' => self . 0 . write_char ('\\') ? , '\n' => return self . 0 . write_str ("\\l") , _ => { } } self . 0 . write_char (c) } }
};
}
