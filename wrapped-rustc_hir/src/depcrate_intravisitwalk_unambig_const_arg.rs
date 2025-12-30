// Generated macro for walk_unambig_const_arg (function)
macro_rules! Depcrate_intravisitwalk_unambig_const_arg {
() => {
// Module: crate::intravisit
// Provides: {"walk_unambig_const_arg"}
// Dependencies: {}
pub fn walk_unambig_const_arg < 'v , V : Visitor < 'v > > (visitor : & mut V , const_arg : & 'v ConstArg < 'v > ,) -> V :: Result { match const_arg . try_as_ambig_ct () { Some (ambig_ct) => visitor . visit_const_arg (ambig_ct) , None => { let ConstArg { hir_id , kind : _ } = const_arg ; visitor . visit_infer (* hir_id , const_arg . span () , InferKind :: Const (const_arg)) } } }
};
}
