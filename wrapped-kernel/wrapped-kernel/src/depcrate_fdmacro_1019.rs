// Generated macro for macro_1019 (macro)
macro_rules! Depcrate_fdmacro_1019 {
() => {
// Module: crate::fd
// Provides: {"macro_1019"}
// Dependencies: {}
bitflags ! { # [doc = " Options for opening files"] # [derive (Debug , Copy , Clone , Default)] pub struct OpenOption : i32 { const O_RDONLY = 0o0000 ; const O_WRONLY = 0o0001 ; const O_RDWR = 0o0002 ; const O_CREAT = 0o0100 ; const O_EXCL = 0o0200 ; const O_TRUNC = 0o1000 ; const O_APPEND = StatusFlags :: O_APPEND . bits () ; const O_NONBLOCK = StatusFlags :: O_NONBLOCK . bits () ; const O_DIRECT = 0o40000 ; const O_DIRECTORY = 0o200_000 ; # [doc = " `O_CLOEXEC` has no functionality in Hermit and will be silently ignored"] const O_CLOEXEC = 0o2_000_000 ; } }
};
}
