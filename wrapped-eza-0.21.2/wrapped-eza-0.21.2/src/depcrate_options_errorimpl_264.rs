// Generated macro for impl_264 (impl)
macro_rules! Depcrate_options_errorimpl_264 {
() => {
// Module: crate::options::error
// Provides: {"impl_264"}
// Dependencies: {}
impl fmt :: Display for OptionsError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use crate :: options :: parser :: TakesValue ; # [rustfmt :: skip] return match self { Self :: BadArgument (arg , attempt) => { if let TakesValue :: Necessary (Some (values)) = arg . takes_value { write ! (f , "Option {} has no {:?} setting ({})" , arg , attempt , Choices (values)) } else { write ! (f , "Option {arg} has no {attempt:?} setting") } } Self :: Parse (e) => write ! (f , "{e}") , Self :: Unsupported (e) => write ! (f , "{e}") , Self :: Conflict (a , b) => write ! (f , "Option {a} conflicts with option {b}") , Self :: Duplicate (a , b) if a == b => write ! (f , "Flag {a} was given twice") , Self :: Duplicate (a , b) => write ! (f , "Flag {a} conflicts with flag {b}") , Self :: Useless (a , false , b) => write ! (f , "Option {a} is useless without option {b}") , Self :: Useless (a , true , b) => write ! (f , "Option {a} is useless given option {b}") , Self :: Useless2 (a , b1 , b2) => write ! (f , "Option {a} is useless without options {b1} or {b2}") , Self :: TreeAllAll => write ! (f , "Option --tree is useless given --all --all") , Self :: FailedParse (s , n , e) => write ! (f , "Value {s:?} not valid for {n}: {e}") , Self :: FailedGlobPattern (ref e) => write ! (f , "Failed to parse glob pattern: {e}") , } ; } }
};
}
