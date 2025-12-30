// Generated macro for len_comparison (function)
macro_rules! Depcrate_missing_asserts_for_indexinglen_comparison {
() => {
// Module: crate::missing_asserts_for_indexing
// Provides: {"len_comparison"}
// Dependencies: {}
# [doc = " Extracts parts out of a length comparison expression."] # [doc = ""] # [doc = " E.g. for `v.len() > 5` this returns `Some((LengthComparison::IntLessThanLength, 5, v.len()))`"] fn len_comparison < 'hir > (bin_op : BinOpKind , left : & 'hir Expr < 'hir > , right : & 'hir Expr < 'hir > ,) -> Option < (LengthComparison , usize , & 'hir Expr < 'hir >) > { macro_rules ! int_lit_pat { ($ id : ident) => { ExprKind :: Lit (Spanned { node : LitKind :: Int (Pu128 ($ id) , _) , .. }) } ; } let (op , left , right) = normalize_comparison (bin_op , left , right) ? ; match (op , left . kind , right . kind) { (Rel :: Lt , int_lit_pat ! (left) , _) => Some ((LengthComparison :: IntLessThanLength , left as usize , right)) , (Rel :: Lt , _ , int_lit_pat ! (right)) => Some ((LengthComparison :: LengthLessThanInt , right as usize , left)) , (Rel :: Le , int_lit_pat ! (left) , _) => Some ((LengthComparison :: IntLessThanOrEqualLength , left as usize , right)) , (Rel :: Le , _ , int_lit_pat ! (right)) => Some ((LengthComparison :: LengthLessThanOrEqualInt , right as usize , left)) , (Rel :: Eq , int_lit_pat ! (left) , _) => Some ((LengthComparison :: LengthEqualInt , left as usize , right)) , (Rel :: Eq , _ , int_lit_pat ! (right)) => Some ((LengthComparison :: LengthEqualInt , right as usize , left)) , _ => None , } }
};
}
