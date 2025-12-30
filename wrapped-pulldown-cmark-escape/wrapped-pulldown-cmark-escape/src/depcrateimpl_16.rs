// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl StrWrite for String { type Error = fmt :: Error ; # [inline] fn write_str (& mut self , s : & str) -> fmt :: Result { self . push_str (s) ; Ok (()) } # [inline] fn write_fmt (& mut self , args : Arguments) -> fmt :: Result { fmt :: Write :: write_fmt (self , args) } }
};
}
