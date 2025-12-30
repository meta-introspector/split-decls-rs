// Generated macro for impl_8048 (impl)
macro_rules! Depcrate_non_expressive_namesimpl_8048 {
() => {
// Module: crate::non_expressive_names
// Provides: {"impl_8048"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for SimilarNamesLocalVisitor < '_ , 'tcx > { fn visit_local (& mut self , local : & 'tcx Local) { if let Some ((init , els)) = & local . kind . init_else_opt () { self . apply (| this | walk_expr (this , init)) ; if let Some (els) = els { self . apply (| this | walk_block (this , els)) ; } } SimilarNamesNameVisitor (self) . visit_pat (& local . pat) ; } fn visit_block (& mut self , blk : & 'tcx Block) { self . single_char_names . push (vec ! []) ; self . apply (| this | walk_block (this , blk)) ; self . check_single_char_names () ; self . single_char_names . pop () ; } fn visit_arm (& mut self , arm : & 'tcx Arm) { self . single_char_names . push (vec ! []) ; self . apply (| this | { SimilarNamesNameVisitor (this) . visit_pat (& arm . pat) ; if let Some (body) = & arm . body { this . apply (| this | walk_expr (this , body)) ; } }) ; self . check_single_char_names () ; self . single_char_names . pop () ; } fn visit_item (& mut self , _ : & Item) { } }
};
}
