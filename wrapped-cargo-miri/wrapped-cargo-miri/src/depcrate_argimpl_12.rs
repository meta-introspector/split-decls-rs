// Generated macro for impl_12 (impl)
macro_rules! Depcrate_argimpl_12 {
() => {
// Module: crate::arg
// Provides: {"impl_12"}
// Dependencies: {}
impl ArgFlagValueIter { pub fn from_string_iter < 'a , I : Iterator < Item = String > + 'a > (args : I , name : & 'a str ,) -> impl Iterator < Item = String > + 'a { ArgSplitFlagValue :: from_string_iter (args , name) . filter_map (Result :: ok) } }
};
}
