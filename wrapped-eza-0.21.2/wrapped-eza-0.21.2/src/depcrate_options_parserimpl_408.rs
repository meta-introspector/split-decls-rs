// Generated macro for impl_408 (impl)
macro_rules! Depcrate_options_parserimpl_408 {
() => {
// Module: crate::options::parser
// Provides: {"impl_408"}
// Dependencies: {}
impl fmt :: Display for Flag { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { match self { Self :: Short (short) => write ! (f , "-{}" , * short as char) , Self :: Long (long) => write ! (f , "--{long}") , } } }
};
}
