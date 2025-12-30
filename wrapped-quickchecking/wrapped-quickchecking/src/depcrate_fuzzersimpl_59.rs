// Generated macro for impl_59 (impl)
macro_rules! Depcrate_fuzzersimpl_59 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_59"}
// Dependencies: {}
# [doc = " Enables to string and format for `ParameterListC` types."] impl fmt :: Display for ParameterListC { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for (i , p) in self . params . iter () . enumerate () { match i { 0 => write ! (f , "{p}") ? , _ => write ! (f , ",{p}") ? , } } Ok (()) } }
};
}
