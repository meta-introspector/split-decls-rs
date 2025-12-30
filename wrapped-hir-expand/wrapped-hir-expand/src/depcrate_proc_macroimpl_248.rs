// Generated macro for impl_248 (impl)
macro_rules! Depcrate_proc_macroimpl_248 {
() => {
// Module: crate::proc_macro
// Provides: {"impl_248"}
// Dependencies: {}
impl FromIterator < (CrateBuilderId , ProcMacroLoadResult) > for ProcMacrosBuilder { fn from_iter < T : IntoIterator < Item = (CrateBuilderId , ProcMacroLoadResult) > > (iter : T) -> Self { let mut builder = ProcMacrosBuilder :: default () ; for (k , v) in iter { builder . insert (k , v) ; } builder } }
};
}
