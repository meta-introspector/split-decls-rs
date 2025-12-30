// Generated macro for impl_1779 (impl)
macro_rules! Depcrate_query_source_aliasing_aliased_fieldimpl_1779 {
() => {
// Module: crate::query_source::aliasing::aliased_field
// Provides: {"impl_1779"}
// Dependencies: {}
impl < S , C , T > EqAll < T > for AliasedField < S , C > where S : AliasSource , C : Column < Table = S :: Target > , Self : ExpressionMethods , < Self as Expression > :: SqlType : sql_types :: SqlType , T : AsExpression < < Self as Expression > :: SqlType > , dsl :: Eq < Self , T > : Expression < SqlType = sql_types :: Bool > , { type Output = dsl :: Eq < Self , T > ; fn eq_all (self , rhs : T) -> Self :: Output { self . eq (rhs) } }
};
}
