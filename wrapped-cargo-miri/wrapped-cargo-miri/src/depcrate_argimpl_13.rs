// Generated macro for impl_13 (impl)
macro_rules! Depcrate_argimpl_13 {
() => {
// Module: crate::arg
// Provides: {"impl_13"}
// Dependencies: {}
impl ArgFlagValueIter { pub fn from_str_iter < 'x : 'a , 'a , I : Iterator < Item = & 'x str > + 'a > (args : I , name : & 'a str ,) -> impl Iterator < Item = & 'x str > + 'a { ArgSplitFlagValue :: from_str_iter (args , name) . filter_map (Result :: ok) } }
};
}
