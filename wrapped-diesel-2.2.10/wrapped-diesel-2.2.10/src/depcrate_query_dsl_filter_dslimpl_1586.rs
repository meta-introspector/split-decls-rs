// Generated macro for impl_1586 (impl)
macro_rules! Depcrate_query_dsl_filter_dslimpl_1586 {
() => {
// Module: crate::query_dsl::filter_dsl
// Provides: {"impl_1586"}
// Dependencies: {}
impl < T , PK > FindDsl < PK > for T where T : Table + FilterDsl < < < T as Table > :: PrimaryKey as EqAll < PK > > :: Output > , T :: PrimaryKey : EqAll < PK > , { type Output = Filter < Self , < T :: PrimaryKey as EqAll < PK > > :: Output > ; fn find (self , id : PK) -> Self :: Output { let primary_key = self . primary_key () ; self . filter (primary_key . eq_all (id)) } }
};
}
