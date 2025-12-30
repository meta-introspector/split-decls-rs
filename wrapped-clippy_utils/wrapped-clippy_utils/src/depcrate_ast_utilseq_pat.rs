// Generated macro for eq_pat (function)
macro_rules! Depcrate_ast_utilseq_pat {
() => {
// Module: crate::ast_utils
// Provides: {"eq_pat"}
// Dependencies: {}
pub fn eq_pat (l : & Pat , r : & Pat) -> bool { use PatKind :: * ; match (& l . kind , & r . kind) { (Missing , _) | (_ , Missing) => unreachable ! () , (Paren (l) , _) => eq_pat (l , r) , (_ , Paren (r)) => eq_pat (l , r) , (Wild , Wild) | (Rest , Rest) => true , (Expr (l) , Expr (r)) => eq_expr (l , r) , (Ident (b1 , i1 , s1) , Ident (b2 , i2 , s2)) => { b1 == b2 && eq_id (* i1 , * i2) && both (s1 . as_deref () , s2 . as_deref () , eq_pat) } , (Range (lf , lt , le) , Range (rf , rt , re)) => { eq_expr_opt (lf . as_deref () , rf . as_deref ()) && eq_expr_opt (lt . as_deref () , rt . as_deref ()) && eq_range_end (& le . node , & re . node) } , (Box (l) , Box (r)) => eq_pat (l , r) , (Ref (l , l_pin , l_mut) , Ref (r , r_pin , r_mut)) => l_pin == r_pin && l_mut == r_mut && eq_pat (l , r) , (Tuple (l) , Tuple (r)) | (Slice (l) , Slice (r)) => over (l , r , eq_pat) , (Path (lq , lp) , Path (rq , rp)) => both (lq . as_deref () , rq . as_deref () , eq_qself) && eq_path (lp , rp) , (TupleStruct (lqself , lp , lfs) , TupleStruct (rqself , rp , rfs)) => { eq_maybe_qself (lqself . as_deref () , rqself . as_deref ()) && eq_path (lp , rp) && over (lfs , rfs , eq_pat) } , (Struct (lqself , lp , lfs , lr) , Struct (rqself , rp , rfs , rr)) => { lr == rr && eq_maybe_qself (lqself . as_deref () , rqself . as_deref ()) && eq_path (lp , rp) && unordered_over (lfs , rfs , eq_field_pat) } , (Or (ls) , Or (rs)) => unordered_over (ls , rs , eq_pat) , (MacCall (l) , MacCall (r)) => eq_mac_call (l , r) , _ => false , } }
};
}
