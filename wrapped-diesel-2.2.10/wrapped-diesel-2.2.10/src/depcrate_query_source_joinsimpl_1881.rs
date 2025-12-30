// Generated macro for impl_1881 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1881 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1881"}
// Dependencies: {}
impl < T , Left , Right , Kind > AppearsInFromClause < T > for Join < Left , Right , Kind > where Left : AppearsInFromClause < T > + QuerySource , Right : AppearsInFromClause < T > + QuerySource , Left :: Count : Plus < Right :: Count > , { type Count = < Left :: Count as Plus < Right :: Count > > :: Output ; }
};
}
