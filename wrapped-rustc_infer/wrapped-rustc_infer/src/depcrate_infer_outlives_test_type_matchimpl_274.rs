// Generated macro for impl_274 (impl)
macro_rules! Depcrate_infer_outlives_test_type_matchimpl_274 {
() => {
// Module: crate::infer::outlives::test_type_match
// Provides: {"impl_274"}
// Dependencies: {}
impl < 'tcx > MatchAgainstHigherRankedOutlives < 'tcx > { # [doc = " Creates the \"Error\" variant that signals \"no match\"."] fn no_match < T > (& self) -> RelateResult < 'tcx , T > { Err (TypeError :: Mismatch) } # [doc = " Binds the pattern variable `br` to `value`; returns an `Err` if the pattern"] # [doc = " is already bound to a different value."] # [instrument (level = "trace" , skip (self))] fn bind (& mut self , br : ty :: BoundRegion , value : ty :: Region < 'tcx > ,) -> RelateResult < 'tcx , ty :: Region < 'tcx > > { match self . map . entry (br) { Entry :: Occupied (entry) => { if * entry . get () == value { Ok (value) } else { self . no_match () } } Entry :: Vacant (entry) => { entry . insert (value) ; Ok (value) } } } }
};
}
