// Generated macro for walk_path (function)
macro_rules! Depcrate_intravisitwalk_path {
() => {
// Module: crate::intravisit
// Provides: {"walk_path"}
// Dependencies: {}
pub fn walk_path < 'v , V : Visitor < 'v > > (visitor : & mut V , path : & Path < 'v >) -> V :: Result { let Path { segments , span : _ , res : _ } = path ; walk_list ! (visitor , visit_path_segment , * segments) ; V :: Result :: output () }
};
}
