// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
# [cfg (not (any (feature = "sysroot-abi" , rust_analyzer)))] fn run (_ : ProtocolFormat) -> std :: io :: Result < () > { Err (std :: io :: Error :: new (std :: io :: ErrorKind :: Unsupported , "proc-macro-srv-cli needs to be compiled with the `sysroot-abi` feature to function" . to_owned () ,)) }
};
}
