// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < W > StrWrite for FmtWriter < W > where W : fmt :: Write , { type Error = fmt :: Error ; # [inline] fn write_str (& mut self , s : & str) -> fmt :: Result { self . 0 . write_str (s) } # [inline] fn write_fmt (& mut self , args : Arguments) -> fmt :: Result { self . 0 . write_fmt (args) } }
};
}
