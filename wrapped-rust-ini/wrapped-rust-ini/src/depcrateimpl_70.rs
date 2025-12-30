// Generated macro for impl_70 (impl)
macro_rules! Depcrateimpl_70 {
() => {
// Module: crate
// Provides: {"impl_70"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { Error :: Io (ref err) => err . fmt (f) , Error :: Parse (ref err) => err . fmt (f) , } } }
};
}
