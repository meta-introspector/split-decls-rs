// Generated macro for impl_1827 (impl)
macro_rules! Depcrate_query_source_aliasing_joinsimpl_1827 {
() => {
// Module: crate::query_source::aliasing::joins
// Provides: {"impl_1827"}
// Dependencies: {}
impl < T , S > JoinTo < T > for Alias < S > where T : Table , S : AliasSource + Default , S :: Target : JoinTo < T > , < S :: Target as JoinTo < T > > :: OnClause : FieldAliasMapper < S > , { type FromClause = < S :: Target as JoinTo < T > > :: FromClause ; type OnClause = < < S :: Target as JoinTo < T > > :: OnClause as FieldAliasMapper < S > > :: Out ; fn join_target (rhs : T) -> (Self :: FromClause , Self :: OnClause) { let (from_clause , on_clause) = < S :: Target as JoinTo < T > > :: join_target (rhs) ; (from_clause , Self :: default () . fields (on_clause)) } }
};
}
