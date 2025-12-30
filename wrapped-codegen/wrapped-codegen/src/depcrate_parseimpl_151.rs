// Generated macro for impl_151 (impl)
macro_rules! Depcrate_parseimpl_151 {
() => {
// Module: crate::parse
// Provides: {"impl_151"}
// Dependencies: {}
impl Display for LoadFileError { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "{path}:{line}:{column}: {error}" , path = self . path . display () , line = self . line , column = self . column , error = self . error ,) } }
};
}
