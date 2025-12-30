// Generated macro for impl_12 (impl)
macro_rules! Depcrate_errorimpl_12 {
() => {
// Module: crate::error
// Provides: {"impl_12"}
// Dependencies: {}
impl std :: fmt :: Display for FromEnvError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match & self . inner { FromEnvErrorInner :: NoEnvVar => write ! (f , "there is no environment variable that describes jobserver to inherit") , FromEnvErrorInner :: NoJobserver => write ! (f , "there is no `--jobserver-fds=` or `--jobserver-auth=` in the environment variable") , FromEnvErrorInner :: CannotParse (s) => write ! (f , "cannot parse jobserver environment variable value: {s}") , FromEnvErrorInner :: CannotOpenPath (s , err) => write ! (f , "cannot open path or name {s} from the jobserver environment variable value: {err}") , FromEnvErrorInner :: CannotOpenFd (fd , err) => write ! (f , "cannot open file descriptor {fd} from the jobserver environment variable value: {err}") , FromEnvErrorInner :: NegativeFd (fd) => write ! (f , "file descriptor {fd} from the jobserver environment variable value is negative") , FromEnvErrorInner :: NotAPipe (fd , None) => write ! (f , "file descriptor {fd} from the jobserver environment variable value is not a pipe") , FromEnvErrorInner :: NotAPipe (fd , Some (err)) => write ! (f , "file descriptor {fd} from the jobserver environment variable value is not a pipe: {err}") , FromEnvErrorInner :: Unsupported => write ! (f , "jobserver inheritance is not supported on this platform") , } } }
};
}
