// Generated macro for eq_opt_coroutine_kind (function)
macro_rules! Depcrate_ast_utilseq_opt_coroutine_kind {
() => {
// Module: crate::ast_utils
// Provides: {"eq_opt_coroutine_kind"}
// Dependencies: {}
fn eq_opt_coroutine_kind (l : Option < CoroutineKind > , r : Option < CoroutineKind >) -> bool { matches ! ((l , r) , (Some (CoroutineKind :: Async { .. }) , Some (CoroutineKind :: Async { .. })) | (Some (CoroutineKind :: Gen { .. }) , Some (CoroutineKind :: Gen { .. })) | (Some (CoroutineKind :: AsyncGen { .. }) , Some (CoroutineKind :: AsyncGen { .. })) | (None , None)) }
};
}
