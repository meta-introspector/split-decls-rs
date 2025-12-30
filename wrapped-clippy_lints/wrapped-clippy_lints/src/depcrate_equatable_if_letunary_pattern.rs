// Generated macro for unary_pattern (function)
macro_rules! Depcrate_equatable_if_letunary_pattern {
() => {
// Module: crate::equatable_if_let
// Provides: {"unary_pattern"}
// Dependencies: {}
# [doc = " detects if pattern matches just one thing"] fn unary_pattern (pat : & Pat < '_ >) -> bool { fn array_rec (pats : & [Pat < '_ >]) -> bool { pats . iter () . all (unary_pattern) } match & pat . kind { PatKind :: Missing => unreachable ! () , PatKind :: Slice (_ , _ , _) | PatKind :: Range (_ , _ , _) | PatKind :: Binding (..) | PatKind :: Wild | PatKind :: Never | PatKind :: Or (_) | PatKind :: Err (_) => false , PatKind :: Struct (_ , a , etc) => etc . is_none () && a . iter () . all (| x | unary_pattern (x . pat)) , PatKind :: Tuple (a , etc) | PatKind :: TupleStruct (_ , a , etc) => etc . as_opt_usize () . is_none () && array_rec (a) , PatKind :: Ref (x , _ , _) | PatKind :: Box (x) | PatKind :: Deref (x) | PatKind :: Guard (x , _) => unary_pattern (x) , PatKind :: Expr (_) => true , } }
};
}
