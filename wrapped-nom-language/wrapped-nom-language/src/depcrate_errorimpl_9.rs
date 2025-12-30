// Generated macro for impl_9 (impl)
macro_rules! Depcrate_errorimpl_9 {
() => {
// Module: crate::error
// Provides: {"impl_9"}
// Dependencies: {}
impl < I : fmt :: Display > fmt :: Display for VerboseError < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "Parse error:") ? ; for (input , error) in & self . errors { match error { VerboseErrorKind :: Nom (e) => writeln ! (f , "{:?} at: {}" , e , input) ? , VerboseErrorKind :: Char (c) => writeln ! (f , "expected '{}' at: {}" , c , input) ? , VerboseErrorKind :: Context (s) => writeln ! (f , "in section '{}', at: {}" , s , input) ? , } } Ok (()) } }
};
}
