// Generated macro for impl_1830 (impl)
macro_rules! Depcrate_query_source_aliasing_joinsimpl_1830 {
() => {
// Module: crate::query_source::aliasing::joins
// Provides: {"impl_1830"}
// Dependencies: {}
impl < S , F , Select , D , W , O , L , Of , G > JoinTo < SelectStatement < FromClause < F > , Select , D , W , O , L , Of , G > > for Alias < S > where F : QuerySource , S : AliasSource + Default , SelectStatement < FromClause < F > , Select , D , W , O , L , Of , G > : JoinTo < Alias < S > > , { type FromClause = SelectStatement < FromClause < F > , Select , D , W , O , L , Of , G > ; type OnClause = < SelectStatement < FromClause < F > , Select , D , W , O , L , Of , G > as JoinTo < Alias < S > > > :: OnClause ; fn join_target (rhs : SelectStatement < FromClause < F > , Select , D , W , O , L , Of , G > ,) -> (Self :: FromClause , Self :: OnClause) { let (_ , on_clause) = diesel :: internal :: table_macro :: SelectStatement :: join_target (Self :: default ()) ; (rhs , on_clause) } }
};
}
