// Generated macro for impl_1828 (impl)
macro_rules! Depcrate_query_source_aliasing_joinsimpl_1828 {
() => {
// Module: crate::query_source::aliasing::joins
// Provides: {"impl_1828"}
// Dependencies: {}
impl < S2 , S > JoinTo < Alias < S2 > > for Alias < S > where S2 : AliasSource , S : AliasSource + Default , S :: Target : JoinTo < Alias < S2 > > , < S :: Target as JoinTo < Alias < S2 > > > :: OnClause : FieldAliasMapper < S > , { type FromClause = < S :: Target as JoinTo < Alias < S2 > > > :: FromClause ; type OnClause = < < S :: Target as JoinTo < Alias < S2 > > > :: OnClause as FieldAliasMapper < S > > :: Out ; fn join_target (rhs : Alias < S2 >) -> (Self :: FromClause , Self :: OnClause) { let (from_clause , on_clause) = < S :: Target as JoinTo < Alias < S2 > > > :: join_target (rhs) ; (from_clause , Self :: default () . fields (on_clause)) } }
};
}
