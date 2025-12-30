// Generated macro for eq_coroutine_kind (function)
macro_rules! Depcrate_ast_utilseq_coroutine_kind {
() => {
// Module: crate::ast_utils
// Provides: {"eq_coroutine_kind"}
// Dependencies: {}
fn eq_coroutine_kind (a : Option < CoroutineKind > , b : Option < CoroutineKind >) -> bool { matches ! ((a , b) , (Some (CoroutineKind :: Async { .. }) , Some (CoroutineKind :: Async { .. })) | (Some (CoroutineKind :: Gen { .. }) , Some (CoroutineKind :: Gen { .. })) | (Some (CoroutineKind :: AsyncGen { .. }) , Some (CoroutineKind :: AsyncGen { .. })) | (None , None)) }
};
}
