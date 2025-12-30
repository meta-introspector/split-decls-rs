// Generated macro for eq_assoc_item_kind (function)
macro_rules! Depcrate_ast_utilseq_assoc_item_kind {
() => {
// Module: crate::ast_utils
// Provides: {"eq_assoc_item_kind"}
// Dependencies: {}
pub fn eq_assoc_item_kind (l : & AssocItemKind , r : & AssocItemKind) -> bool { use AssocItemKind :: * ; match (l , r) { (Const (box ConstItem { defaultness : ld , ident : li , generics : lg , ty : lt , rhs : lb , define_opaque : _ , }) , Const (box ConstItem { defaultness : rd , ident : ri , generics : rg , ty : rt , rhs : rb , define_opaque : _ , }) ,) => { eq_defaultness (* ld , * rd) && eq_id (* li , * ri) && eq_generics (lg , rg) && eq_ty (lt , rt) && both (lb . as_ref () , rb . as_ref () , eq_const_item_rhs) } , (Fn (box ast :: Fn { defaultness : ld , sig : lf , ident : li , generics : lg , contract : lc , body : lb , define_opaque : _ , }) , Fn (box ast :: Fn { defaultness : rd , sig : rf , ident : ri , generics : rg , contract : rc , body : rb , define_opaque : _ , }) ,) => { eq_defaultness (* ld , * rd) && eq_fn_sig (lf , rf) && eq_id (* li , * ri) && eq_generics (lg , rg) && eq_opt_fn_contract (lc , rc) && both (lb . as_ref () , rb . as_ref () , | l , r | eq_block (l , r)) } , (Type (box TyAlias { defaultness : ld , ident : li , generics : lg , after_where_clause : lw , bounds : lb , ty : lt , }) , Type (box TyAlias { defaultness : rd , ident : ri , generics : rg , after_where_clause : rw , bounds : rb , ty : rt , }) ,) => { eq_defaultness (* ld , * rd) && eq_id (* li , * ri) && eq_generics (lg , rg) && over (& lw . predicates , & rw . predicates , eq_where_predicate) && over (lb , rb , eq_generic_bound) && both (lt . as_ref () , rt . as_ref () , | l , r | eq_ty (l , r)) } , (MacCall (l) , MacCall (r)) => eq_mac_call (l , r) , _ => false , } }
};
}
