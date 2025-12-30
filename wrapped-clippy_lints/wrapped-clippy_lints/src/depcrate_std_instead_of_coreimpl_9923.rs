// Generated macro for impl_9923 (impl)
macro_rules! Depcrate_std_instead_of_coreimpl_9923 {
() => {
// Module: crate::std_instead_of_core
// Provides: {"impl_9923"}
// Dependencies: {}
impl StdReexports { pub fn new (conf : & 'static Conf) -> Self { Self { lint_points : Option :: default () , msrv : conf . msrv , } } fn lint_if_finish (& mut self , cx : & LateContext < '_ > , krate : Span , lint_point : LintPoint) { match & mut self . lint_points { Some ((prev_krate , prev_lints)) if prev_krate . overlaps (krate) => { prev_lints . push (lint_point) ; } , _ => emit_lints (cx , self . lint_points . replace ((krate , vec ! [lint_point]))) , } } }
};
}
