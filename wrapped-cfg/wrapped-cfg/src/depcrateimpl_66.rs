// Generated macro for impl_66 (impl)
macro_rules! Depcrateimpl_66 {
() => {
// Module: crate
// Provides: {"impl_66"}
// Dependencies: {}
impl FromIterator < CfgAtom > for CfgOptions { fn from_iter < T : IntoIterator < Item = CfgAtom > > (iter : T) -> Self { let mut options = CfgOptions :: default () ; options . extend (iter) ; options } }
};
}
