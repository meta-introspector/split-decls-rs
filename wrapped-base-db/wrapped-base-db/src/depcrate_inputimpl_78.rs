// Generated macro for impl_78 (impl)
macro_rules! Depcrate_inputimpl_78 {
() => {
// Module: crate::input
// Provides: {"impl_78"}
// Dependencies: {}
impl FromIterator < (String , String) > for Env { fn from_iter < T : IntoIterator < Item = (String , String) > > (iter : T) -> Self { Env { entries : FromIterator :: from_iter (iter) } } }
};
}
