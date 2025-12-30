// Generated macro for impl_71 (impl)
macro_rules! Depcrateimpl_71 {
() => {
// Module: crate
// Provides: {"impl_71"}
// Dependencies: {}
impl error :: Error for Error { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match * self { Error :: Io (ref err) => err . source () , Error :: Parse (ref err) => err . source () , } } }
};
}
