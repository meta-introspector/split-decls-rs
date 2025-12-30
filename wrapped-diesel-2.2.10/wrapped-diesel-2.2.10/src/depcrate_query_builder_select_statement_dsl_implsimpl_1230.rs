// Generated macro for impl_1230 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1230 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1230"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC , PK > FindDsl < PK > for SelectStatement < FromClause < F > , S , D , W , O , LOf , G , H , LC > where F : Table , F :: PrimaryKey : EqAll < PK > , Self : FilterDsl < < F :: PrimaryKey as EqAll < PK > > :: Output > , { type Output = Filter < Self , < F :: PrimaryKey as EqAll < PK > > :: Output > ; fn find (self , id : PK) -> Self :: Output { let primary_key = self . from . source . primary_key () ; FilterDsl :: filter (self , primary_key . eq_all (id)) } }
};
}
