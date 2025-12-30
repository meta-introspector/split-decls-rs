// Generated macro for impl_571 (impl)
macro_rules! Depcrate_se_simple_typeimpl_571 {
() => {
// Module: crate::se::simple_type
// Provides: {"impl_571"}
// Dependencies: {}
impl < W : Write > AtomicSerializer < W > { fn write_delimiter (& mut self) -> fmt :: Result { if self . write_delimiter { return self . writer . write_char (' ') ; } Ok (()) } fn write_str (& mut self , value : & str) -> Result < () , SeError > { self . write_delimiter () ? ; Ok (self . writer . write_str (value) ?) } fn write_fmt (& mut self , args : fmt :: Arguments < '_ >) -> Result < () , SeError > { self . write_delimiter () ? ; Ok (self . writer . write_fmt (args) ?) } }
};
}
