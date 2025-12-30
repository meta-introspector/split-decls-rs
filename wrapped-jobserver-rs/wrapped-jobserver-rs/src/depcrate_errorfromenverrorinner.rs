// Generated macro for FromEnvErrorInner (enum)
macro_rules! Depcrate_errorFromEnvErrorInner {
() => {
// Module: crate::error
// Provides: {"FromEnvErrorInner"}
// Dependencies: {}
# [allow (dead_code)] # [derive (Debug)] pub (crate) enum FromEnvErrorInner { NoEnvVar , NoJobserver , CannotParse (String) , CannotOpenPath (String , std :: io :: Error) , CannotOpenFd (RawFd , std :: io :: Error) , NegativeFd (RawFd) , NotAPipe (RawFd , Option < std :: io :: Error >) , Unsupported , }
};
}
