// Generated macro for eq_foreign_item_kind (function)
macro_rules! Depcrate_ast_utilseq_foreign_item_kind {
() => {
// Module: crate::ast_utils
// Provides: {"eq_foreign_item_kind"}
// Dependencies: {}
pub fn eq_foreign_item_kind (l : & ForeignItemKind , r : & ForeignItemKind) -> bool { use ForeignItemKind :: * ; match (l , r) { (Static (box StaticItem { ident : li , ty : lt , mutability : lm , expr : le , safety : ls , define_opaque : _ , }) , Static (box StaticItem { ident : ri , ty : rt , mutability : rm , expr : re , safety : rs , define_opaque : _ , }) ,) => eq_id (* li , * ri) && eq_ty (lt , rt) && lm == rm && eq_expr_opt (le . as_deref () , re . as_deref ()) && ls == rs , (Fn (box ast :: Fn { defaultness : ld , sig : lf , ident : li , generics : lg , contract : lc , body : lb , define_opaque : _ , }) , Fn (box ast :: Fn { defaultness : rd , sig : rf , ident : ri , generics : rg , contract : rc , body : rb , define_opaque : _ , }) ,) => { eq_defaultness (* ld , * rd) && eq_fn_sig (lf , rf) && eq_id (* li , * ri) && eq_generics (lg , rg) && eq_opt_fn_contract (lc , rc) && both (lb . as_ref () , rb . as_ref () , | l , r | eq_block (l , r)) } , (TyAlias (box ast :: TyAlias { defaultness : ld , ident : li , generics : lg , after_where_clause : lw , bounds : lb , ty : lt , }) , TyAlias (box ast :: TyAlias { defaultness : rd , ident : ri , generics : rg , after_where_clause : rw , bounds : rb , ty : rt , }) ,) => { eq_defaultness (* ld , * rd) && eq_id (* li , * ri) && eq_generics (lg , rg) && over (& lw . predicates , & rw . predicates , eq_where_predicate) && over (lb , rb , eq_generic_bound) && both (lt . as_ref () , rt . as_ref () , | l , r | eq_ty (l , r)) } , (MacCall (l) , MacCall (r)) => eq_mac_call (l , r) , _ => false , } }
};
}
