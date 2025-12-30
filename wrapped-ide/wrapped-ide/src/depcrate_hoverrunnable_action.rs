// Generated macro for runnable_action (function)
macro_rules! Depcrate_hoverrunnable_action {
() => {
// Module: crate::hover
// Provides: {"runnable_action"}
// Dependencies: {}
fn runnable_action (sema : & hir :: Semantics < '_ , RootDatabase > , def : Definition , file_id : FileId ,) -> Option < HoverAction > { match def { Definition :: Module (it) => runnable_mod (sema , it) . map (HoverAction :: Runnable) , Definition :: Function (func) => { let src = func . source (sema . db) ? ; if src . file_id . file_id () . is_none_or (| f | f . file_id (sema . db) != file_id) { cov_mark :: hit ! (hover_macro_generated_struct_fn_doc_comment) ; cov_mark :: hit ! (hover_macro_generated_struct_fn_doc_attr) ; return None ; } runnable_fn (sema , func) . map (HoverAction :: Runnable) } _ => None , } }
};
}
