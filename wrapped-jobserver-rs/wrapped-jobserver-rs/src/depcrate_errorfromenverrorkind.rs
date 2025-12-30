// Generated macro for FromEnvErrorKind (enum)
macro_rules! Depcrate_errorFromEnvErrorKind {
() => {
// Module: crate::error
// Provides: {"FromEnvErrorKind"}
// Dependencies: {}
# [doc = " Kind of an error returned from [`Client::from_env_ext`] function."] # [doc = ""] # [doc = " [`Client::from_env_ext`]: crate::Client::from_env_ext"] # [derive (Debug)] # [non_exhaustive] pub enum FromEnvErrorKind { # [doc = " There is no environment variable that describes jobserver to inherit."] NoEnvVar , # [doc = " There is no jobserver in the environment variable."] # [doc = " Variables associated with Make can be used for passing data other than jobserver info."] NoJobserver , # [doc = " Cannot parse jobserver environment variable value, incorrect format."] CannotParse , # [doc = " Cannot open path or name from the jobserver environment variable value."] CannotOpenPath , # [doc = " Cannot open file descriptor from the jobserver environment variable value."] CannotOpenFd , # [doc = " The jobserver style is a simple pipe, but at least one of the file descriptors"] # [doc = " is negative, which means it is disabled for this process"] # [doc = " ([GNU `make` manual: POSIX Jobserver Interaction](https://www.gnu.org/software/make/manual/make.html#POSIX-Jobserver))."] NegativeFd , # [doc = " File descriptor from the jobserver environment variable value is not a pipe."] NotAPipe , # [doc = " Jobserver inheritance is not supported on this platform."] Unsupported , }
};
}
