// Generated macro for impl_1777 (impl)
macro_rules! Depcrate_query_source_aliasing_aliased_fieldimpl_1777 {
() => {
// Module: crate::query_source::aliasing::aliased_field
// Provides: {"impl_1777"}
// Dependencies: {}
impl < S , C1 , C2 > ValidGrouping < AliasedField < S , C1 > > for AliasedField < S , C2 > where S : AliasSource , C1 : Column < Table = S :: Target > , C2 : Column < Table = S :: Target > , C2 : ValidGrouping < C1 , IsAggregate = is_aggregate :: Yes > , { type IsAggregate = is_aggregate :: Yes ; }
};
}
