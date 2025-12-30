// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl Extend < CfgAtom > for CfgOptions { fn extend < T : IntoIterator < Item = CfgAtom > > (& mut self , iter : T) { iter . into_iter () . for_each (| cfg_flag | self . insert_any_atom (cfg_flag)) ; } }
};
}
