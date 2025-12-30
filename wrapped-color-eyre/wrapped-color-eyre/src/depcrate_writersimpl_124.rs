// Generated macro for impl_124 (impl)
macro_rules! Depcrate_writersimpl_124 {
() => {
// Module: crate::writers
// Provides: {"impl_124"}
// Dependencies: {}
impl < H : ? Sized , W > fmt :: Write for ReadyHeaderWriter < '_ , '_ , H , W > where H : Display , W : fmt :: Write , { fn write_str (& mut self , s : & str) -> fmt :: Result { if ! self . 0 . started && ! s . is_empty () { self . 0 . inner . write_fmt (format_args ! ("{}" , self . 0 . header)) ? ; self . 0 . started = true ; } self . 0 . inner . write_str (s) } }
};
}
