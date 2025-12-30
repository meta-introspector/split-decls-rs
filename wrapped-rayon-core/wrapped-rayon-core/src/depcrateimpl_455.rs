// Generated macro for impl_455 (impl)
macro_rules! Depcrateimpl_455 {
() => {
// Module: crate
// Provides: {"impl_455"}
// Dependencies: {}
impl ThreadPoolBuildError { fn new (kind : ErrorKind) -> ThreadPoolBuildError { ThreadPoolBuildError { kind } } fn is_unsupported (& self) -> bool { matches ! (& self . kind , ErrorKind :: IOError (e) if e . kind () == io :: ErrorKind :: Unsupported) } }
};
}
