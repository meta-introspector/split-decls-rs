// Generated macro for impl_407 (impl)
macro_rules! Depcrate_options_parserimpl_407 {
() => {
// Module: crate::options::parser
// Provides: {"impl_407"}
// Dependencies: {}
impl Flag { pub fn matches (& self , arg : & Arg) -> bool { match self { Self :: Short (short) => arg . short == Some (* short) , Self :: Long (long) => arg . long == * long , } } }
};
}
