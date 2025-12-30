// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: NoArguments => write ! (f , "No arguments provided") , Self :: UnrecognizedArgument (arg) => write ! (f , "Unrecognized argument: '{arg}'") , Self :: MissingToolchainVersion => write ! (f , "No toolchain version specified after '--version'") , Self :: UnrecognizedToolchain (name) => write ! (f , "Unrecognized toolchain name: `{name}` (options are 'msrv', 'stable', and 'nightly')") } } }
};
}
