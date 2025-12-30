// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: FormatFailed (e) => e . fmt (f) , Self :: UnexpectedEvent => f . write_str ("Unexpected event while reconstructing Markdown") , } } }
};
}
