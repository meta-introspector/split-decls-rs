// Generated macro for impl_46 (impl)
macro_rules! Depcrate_termimpl_46 {
() => {
// Module: crate::term
// Provides: {"impl_46"}
// Dependencies: {}
# [cfg (any (unix , all (target_os = "wasi" , target_env = "p1")))] impl AsRawFd for Term { fn as_raw_fd (& self) -> RawFd { match self . inner . target { TermTarget :: Stdout => libc :: STDOUT_FILENO , TermTarget :: Stderr => libc :: STDERR_FILENO , # [cfg (unix)] TermTarget :: ReadWritePair (ReadWritePair { ref write , .. }) => { write . lock () . unwrap () . as_raw_fd () } } } }
};
}
