// Generated macro for is_thread_local_initializer (function)
macro_rules! Depcrate_missing_const_for_thread_localis_thread_local_initializer {
() => {
// Module: crate::missing_const_for_thread_local
// Provides: {"is_thread_local_initializer"}
// Dependencies: {}
# [inline] fn is_thread_local_initializer (cx : & LateContext < '_ > , fn_kind : intravisit :: FnKind < '_ > , span : rustc_span :: Span ,) -> Option < bool > { let macro_def_id = span . source_callee () ? . macro_def_id ? ; Some (cx . tcx . is_diagnostic_item (thread_local_macro , macro_def_id) && matches ! (fn_kind , intravisit :: FnKind :: ItemFn (..)) ,) }
};
}
