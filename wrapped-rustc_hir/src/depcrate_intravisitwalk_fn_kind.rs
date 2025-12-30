// Generated macro for walk_fn_kind (function)
macro_rules! Depcrate_intravisitwalk_fn_kind {
() => {
// Module: crate::intravisit
// Provides: {"walk_fn_kind"}
// Dependencies: {}
pub fn walk_fn_kind < 'v , V : Visitor < 'v > > (visitor : & mut V , function_kind : FnKind < 'v >) -> V :: Result { match function_kind { FnKind :: ItemFn (_ , generics , ..) => { try_visit ! (visitor . visit_generics (generics)) ; } FnKind :: Closure | FnKind :: Method (..) => { } } V :: Result :: output () }
};
}
