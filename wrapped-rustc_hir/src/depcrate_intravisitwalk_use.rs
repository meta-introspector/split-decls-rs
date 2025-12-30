// Generated macro for walk_use (function)
macro_rules! Depcrate_intravisitwalk_use {
() => {
// Module: crate::intravisit
// Provides: {"walk_use"}
// Dependencies: {}
pub fn walk_use < 'v , V : Visitor < 'v > > (visitor : & mut V , path : & 'v UsePath < 'v > , hir_id : HirId ,) -> V :: Result { let UsePath { segments , ref res , span } = * path ; for res in res . present_items () { try_visit ! (visitor . visit_path (& Path { segments , res , span } , hir_id)) ; } V :: Result :: output () }
};
}
