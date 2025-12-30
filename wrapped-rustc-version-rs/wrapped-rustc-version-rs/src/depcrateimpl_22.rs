// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl fmt :: Display for LlvmVersionParseError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { LlvmVersionParseError :: ParseIntError (e) => { write ! (f , "error parsing LLVM version component: {}" , e) } LlvmVersionParseError :: ComponentMustNotHaveLeadingZeros => { write ! (f , "a version component must not have leading zeros") } LlvmVersionParseError :: ComponentMustNotHaveSign => { write ! (f , "a version component must not have a sign") } LlvmVersionParseError :: MinorVersionMustBeZeroAfter4 => write ! (f , "LLVM's minor version component must be 0 for versions greater than 4.0") , LlvmVersionParseError :: MinorVersionRequiredBefore4 => write ! (f , "LLVM's minor version component is required for versions less than 4.0") , LlvmVersionParseError :: TooManyComponents => write ! (f , "too many version components") , } } }
};
}
