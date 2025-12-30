// Generated macro for impl_11 (impl)
macro_rules! Depcrate_errorimpl_11 {
() => {
// Module: crate::error
// Provides: {"impl_11"}
// Dependencies: {}
impl FromEnvError { # [doc = " Get the error kind."] pub fn kind (& self) -> FromEnvErrorKind { match self . inner { FromEnvErrorInner :: NoEnvVar => FromEnvErrorKind :: NoEnvVar , FromEnvErrorInner :: NoJobserver => FromEnvErrorKind :: NoJobserver , FromEnvErrorInner :: CannotParse (_) => FromEnvErrorKind :: CannotParse , FromEnvErrorInner :: CannotOpenPath (..) => FromEnvErrorKind :: CannotOpenPath , FromEnvErrorInner :: CannotOpenFd (..) => FromEnvErrorKind :: CannotOpenFd , FromEnvErrorInner :: NegativeFd (..) => FromEnvErrorKind :: NegativeFd , FromEnvErrorInner :: NotAPipe (..) => FromEnvErrorKind :: NotAPipe , FromEnvErrorInner :: Unsupported => FromEnvErrorKind :: Unsupported , } } }
};
}
