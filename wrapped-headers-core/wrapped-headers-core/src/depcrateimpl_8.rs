// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { match & self . kind { Kind :: Invalid => f . write_str ("invalid HTTP header") , } } }
};
}
