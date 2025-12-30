// Generated macro for impl_1752 (impl)
macro_rules! Depcrate_query_source_aliasing_aliasimpl_1752 {
() => {
// Module: crate::query_source::aliasing::alias
// Provides: {"impl_1752"}
// Dependencies: {}
impl < T1 , S1 , S2 > AliasAppearsInFromClause < S1 , Alias < S2 > > for T1 where S2 : AliasSource , T1 : AliasAliasAppearsInFromClause < S2 :: Target , S1 , S2 > , { type Count = < T1 as AliasAliasAppearsInFromClause < S2 :: Target , S1 , S2 > > :: Count ; }
};
}
