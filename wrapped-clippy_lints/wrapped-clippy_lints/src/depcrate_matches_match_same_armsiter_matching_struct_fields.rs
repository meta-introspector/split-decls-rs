// Generated macro for iter_matching_struct_fields (function)
macro_rules! Depcrate_matches_match_same_armsiter_matching_struct_fields {
() => {
// Module: crate::matches::match_same_arms
// Provides: {"iter_matching_struct_fields"}
// Dependencies: {}
# [doc = " Iterates over the pairs of fields with matching names."] fn iter_matching_struct_fields < 'a > (left : & 'a [(Symbol , NormalizedPat < 'a >)] , right : & 'a [(Symbol , NormalizedPat < 'a >)] ,) -> impl Iterator < Item = (& 'a NormalizedPat < 'a > , & 'a NormalizedPat < 'a >) > + 'a { struct Iter < 'a > (slice :: Iter < 'a , (Symbol , NormalizedPat < 'a >) > , slice :: Iter < 'a , (Symbol , NormalizedPat < 'a >) > ,) ; impl < 'a > Iterator for Iter < 'a > { type Item = (& 'a NormalizedPat < 'a > , & 'a NormalizedPat < 'a >) ; fn next (& mut self) -> Option < Self :: Item > { let mut left = self . 0 . next () ? ; let mut right = self . 1 . next () ? ; loop { match left . 0 . cmp (& right . 0) { Ordering :: Equal => return Some ((& left . 1 , & right . 1)) , Ordering :: Less => left = self . 0 . next () ? , Ordering :: Greater => right = self . 1 . next () ? , } } } } Iter (left . iter () , right . iter ()) }
};
}
