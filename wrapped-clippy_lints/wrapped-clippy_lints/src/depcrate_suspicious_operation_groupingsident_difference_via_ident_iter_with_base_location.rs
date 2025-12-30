// Generated macro for ident_difference_via_ident_iter_with_base_location (function)
macro_rules! Depcrate_suspicious_operation_groupingsident_difference_via_ident_iter_with_base_location {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"ident_difference_via_ident_iter_with_base_location"}
// Dependencies: {}
fn ident_difference_via_ident_iter_with_base_location < Iterable : Into < IdentIter > > (left : Iterable , right : Iterable , mut base : IdentLocation ,) -> (IdentDifference , IdentLocation) { let mut difference = IdentDifference :: NoDifference ; let mut left_iterator = left . into () ; let mut right_iterator = right . into () ; loop { match (left_iterator . next () , right_iterator . next ()) { (Some (left_ident) , Some (right_ident)) => { if ! eq_id (left_ident , right_ident) { difference += IdentDifference :: Single (base) ; if difference . is_complete () { return (difference , base) ; } } } , (Some (_) , None) | (None , Some (_)) => { return (IdentDifference :: NonIdent , base) ; } , (None , None) => { return (difference , base) ; } , } base += IdentLocation { index : 1 } ; } }
};
}
