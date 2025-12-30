// Generated macro for send_syncify (function)
macro_rules! Depcrate_mock_functionsend_syncify {
() => {
// Module: crate::mock_function
// Provides: {"send_syncify"}
// Dependencies: {}
# [doc = " Add Send + Sync to a where clause"] fn send_syncify (wc : & mut Option < WhereClause > , bounded_ty : Type) { let mut bounds = Punctuated :: new () ; bounds . push (TypeParamBound :: Trait (TraitBound { paren_token : None , modifier : TraitBoundModifier :: None , lifetimes : None , path : Path { leading_colon : Some (< Token ! [::] > :: default ()) , segments : [PathSegment :: from (format_ident ! ("std")) , PathSegment :: from (format_ident ! ("marker")) , PathSegment :: from (format_ident ! ("Send")) ,] . into_iter () . collect () , } })) ; bounds . push (TypeParamBound :: Trait (TraitBound { paren_token : None , modifier : TraitBoundModifier :: None , lifetimes : None , path : Path { leading_colon : Some (< Token ! [::] > :: default ()) , segments : [PathSegment :: from (format_ident ! ("std")) , PathSegment :: from (format_ident ! ("marker")) , PathSegment :: from (format_ident ! ("Sync")) ,] . into_iter () . collect () , } })) ; if wc . is_none () { * wc = Some (WhereClause { where_token : < Token ! [where] > :: default () , predicates : Punctuated :: new () }) ; } wc . as_mut () . unwrap () . predicates . push (WherePredicate :: Type (PredicateType { lifetimes : None , bounded_ty , colon_token : Default :: default () , bounds })) ; }
};
}
