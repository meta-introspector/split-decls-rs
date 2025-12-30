// Generated macro for new_empty_where_type_predicate (function)
macro_rules! Depcrate_expandnew_empty_where_type_predicate {
() => {
// Module: crate::expand
// Provides: {"new_empty_where_type_predicate"}
// Dependencies: {}
# [doc = " Create a `where` predicate for `ident`, without any [bound][TypeParamBound]s yet."] fn new_empty_where_type_predicate (ident : Ident) -> PredicateType { let mut path_segments = Punctuated :: < PathSegment , PathSep > :: new () ; path_segments . push_value (PathSegment { ident , arguments : PathArguments :: None , }) ; PredicateType { lifetimes : None , bounded_ty : Type :: Path (TypePath { qself : None , path : Path { leading_colon : None , segments : path_segments , } , }) , colon_token : Colon { spans : [Span :: call_site ()] , } , bounds : Punctuated :: < TypeParamBound , Plus > :: new () , } }
};
}
