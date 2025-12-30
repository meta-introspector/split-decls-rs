// Generated macro for impl_8 (impl)
macro_rules! Depcrate_writerimpl_8 {
() => {
// Module: crate::writer
// Provides: {"impl_8"}
// Dependencies: {}
impl < W : Write > Write for GenericWriter < W > { fn write_str (& mut self , s : & str) -> fmt :: Result { self . 0 . write_str (s) } fn write_char (& mut self , c : char) -> fmt :: Result { self . 0 . write_char (c) } fn write_fmt (self : & mut Self , args : fmt :: Arguments < '_ >) -> fmt :: Result { self . 0 . write_fmt (args) } }
};
}
