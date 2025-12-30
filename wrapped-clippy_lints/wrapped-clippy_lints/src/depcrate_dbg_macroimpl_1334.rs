// Generated macro for impl_1334 (impl)
macro_rules! Depcrate_dbg_macroimpl_1334 {
() => {
// Module: crate::dbg_macro
// Provides: {"impl_1334"}
// Dependencies: {}
impl DbgMacro { pub fn new (conf : & 'static Conf) -> Self { DbgMacro { allow_dbg_in_tests : conf . allow_dbg_in_tests , checked_dbg_call_site : FxHashSet :: default () , prev_ctxt : SyntaxContext :: root () , } } }
};
}
