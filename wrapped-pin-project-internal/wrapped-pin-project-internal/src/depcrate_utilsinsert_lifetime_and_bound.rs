// Generated macro for insert_lifetime_and_bound (function)
macro_rules! Depcrate_utilsinsert_lifetime_and_bound {
() => {
// Module: crate::utils
// Provides: {"insert_lifetime_and_bound"}
// Dependencies: {}
# [doc = " Like `insert_lifetime`, but also generates a bound of the form"] # [doc = " `OriginalType<A, B>: 'lifetime`. Used when generating the definition"] # [doc = " of a projection type"] pub (crate) fn insert_lifetime_and_bound (generics : & mut Generics , lifetime : Lifetime , orig_generics : & Generics , orig_ident : & Ident ,) -> WherePredicate { insert_lifetime (generics , lifetime . clone ()) ; let orig_type : Type = parse_quote ! (# orig_ident # orig_generics) ; let mut punct = Punctuated :: new () ; punct . push (TypeParamBound :: Lifetime (lifetime)) ; WherePredicate :: Type (PredicateType { lifetimes : None , bounded_ty : orig_type , colon_token : < Token ! [:] > :: default () , bounds : punct , }) }
};
}
