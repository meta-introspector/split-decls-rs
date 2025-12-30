// Generated macro for impl_819 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_819 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_819"}
// Dependencies: {}
impl < 'a , T : ? Sized > fmt :: Write for & 'a Retained < T > where & 'a T : fmt :: Write , { # [inline] fn write_str (& mut self , s : & str) -> fmt :: Result { (& * * * self) . write_str (s) } # [inline] fn write_char (& mut self , c : char) -> fmt :: Result { (& * * * self) . write_char (c) } # [inline] fn write_fmt (& mut self , args : fmt :: Arguments < '_ >) -> fmt :: Result { (& * * * self) . write_fmt (args) } }
};
}
