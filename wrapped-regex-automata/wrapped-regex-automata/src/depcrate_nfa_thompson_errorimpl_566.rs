// Generated macro for impl_566 (impl)
macro_rules! Depcrate_nfa_thompson_errorimpl_566 {
() => {
// Module: crate::nfa::thompson::error
// Provides: {"impl_566"}
// Dependencies: {}
impl core :: fmt :: Display for BuildError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self . kind () { # [cfg (feature = "syntax")] BuildErrorKind :: Syntax (_) => write ! (f , "error parsing regex") , BuildErrorKind :: Captures (_) => { write ! (f , "error with capture groups") } BuildErrorKind :: Word (_) => { write ! (f , "NFA contains Unicode word boundary") } BuildErrorKind :: TooManyPatterns { given , limit } => write ! (f , "attempted to compile {given} patterns, \
                 which exceeds the limit of {limit}" ,) , BuildErrorKind :: TooManyStates { given , limit } => write ! (f , "attempted to compile {given} NFA states, \
                 which exceeds the limit of {limit}" ,) , BuildErrorKind :: ExceededSizeLimit { limit } => write ! (f , "heap usage during NFA compilation exceeded limit of {limit}" ,) , BuildErrorKind :: InvalidCaptureIndex { index } => write ! (f , "capture group index {index} is invalid \
                 (too big or discontinuous)" ,) , # [cfg (feature = "syntax")] BuildErrorKind :: UnsupportedCaptures => write ! (f , "currently captures must be disabled when compiling \
                 a reverse NFA" ,) , } } }
};
}
