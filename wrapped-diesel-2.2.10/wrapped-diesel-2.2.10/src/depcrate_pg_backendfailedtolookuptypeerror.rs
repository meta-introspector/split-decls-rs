// Generated macro for FailedToLookupTypeError (struct)
macro_rules! Depcrate_pg_backendFailedToLookupTypeError {
() => {
// Module: crate::pg::backend
// Provides: {"FailedToLookupTypeError"}
// Dependencies: {}
# [doc = " This error indicates that a type lookup for a custom"] # [doc = " postgres type failed"] # [derive (Clone , Debug , Hash , PartialEq , Eq)] # [cfg_attr (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))] # [allow (unreachable_pub)] pub struct FailedToLookupTypeError (Box < PgMetadataCacheKey < 'static > >) ;
};
}
