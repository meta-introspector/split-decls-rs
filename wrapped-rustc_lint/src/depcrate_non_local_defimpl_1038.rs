// Generated macro for impl_1038 (impl)
macro_rules! Depcrate_non_local_defimpl_1038 {
() => {
// Module: crate::non_local_def
// Provides: {"impl_1038"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for PathCollector < 'tcx > { fn visit_path (& mut self , path : & Path < 'tcx > , _id : HirId) { self . paths . push (path . clone ()) ; intravisit :: walk_path (self , path) } }
};
}
