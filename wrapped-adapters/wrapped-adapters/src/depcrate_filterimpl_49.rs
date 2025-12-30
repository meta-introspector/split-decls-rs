// Generated macro for impl_49 (impl)
macro_rules! Depcrate_filterimpl_49 {
() => {
// Module: crate::filter
// Provides: {"impl_49"}
// Dependencies: {}
impl < D , F > FilterDataProvider < D , F > where F : Fn (DataIdentifierBorrowed) -> bool , { fn check (& self , marker : DataMarkerInfo , req : DataRequest) -> Result < () , DataError > { if ! (self . predicate) (req . id) { return Err (DataErrorKind :: IdentifierNotFound . with_str_context (self . filter_name) . with_req (marker , req)) ; } Ok (()) } }
};
}
