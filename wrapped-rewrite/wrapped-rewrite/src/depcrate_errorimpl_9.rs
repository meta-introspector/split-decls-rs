// Generated macro for impl_9 (impl)
macro_rules! Depcrate_errorimpl_9 {
() => {
// Module: crate::error
// Provides: {"impl_9"}
// Dependencies: {}
impl Error { # [doc = " Get the kind of error."] pub fn kind (& self) -> ErrorKind { match & self . inner { ErrorInner :: Io (e) => ErrorKind :: Io (e . kind ()) , ErrorInner :: Parse (_) => ErrorKind :: Parse , ErrorInner :: Write (_) => ErrorKind :: Write , ErrorInner :: Modify (_) => ErrorKind :: Modify , } } pub (crate) fn io (error : io :: Error) -> Self { Self { inner : ErrorInner :: Io (error) , } } pub (crate) fn parse (error : build :: Error) -> Self { Self { inner : ErrorInner :: Parse (error) , } } pub (crate) fn write (error : build :: Error) -> Self { Self { inner : ErrorInner :: Write (error) , } } pub (crate) fn modify (message : impl Into < String >) -> Self { Self { inner : ErrorInner :: Modify (message . into ()) , } } }
};
}
