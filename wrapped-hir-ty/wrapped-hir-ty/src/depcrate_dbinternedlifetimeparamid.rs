// Generated macro for InternedLifetimeParamId (struct)
macro_rules! Depcrate_dbInternedLifetimeParamId {
() => {
// Module: crate::db
// Provides: {"InternedLifetimeParamId"}
// Dependencies: {}
# [salsa_macros :: interned (no_lifetime , debug , revisions = usize :: MAX)] # [derive (PartialOrd , Ord)] pub struct InternedLifetimeParamId { # [doc = " This stores the param and its index."] pub loc : (LifetimeParamId , u32) , }
};
}
