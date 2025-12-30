// Generated macro for impl_1794 (impl)
macro_rules! Depcrate_query_source_aliasing_dsl_implsimpl_1794 {
() => {
// Module: crate::query_source::aliasing::dsl_impls
// Provides: {"impl_1794"}
// Dependencies: {}
impl < S , PK > FindDsl < PK > for Alias < S > where S : AliasSource , S :: Target : Table , < S :: Target as Table > :: PrimaryKey : FieldAliasMapper < S > , < < S :: Target as Table > :: PrimaryKey as FieldAliasMapper < S > > :: Out : EqAll < PK > , Self : FilterDsl < < < < S :: Target as Table > :: PrimaryKey as FieldAliasMapper < S > > :: Out as EqAll < PK > > :: Output , > , { type Output = dsl :: Filter < Self , < < < S :: Target as Table > :: PrimaryKey as FieldAliasMapper < S > > :: Out as EqAll < PK > > :: Output , > ; fn find (self , id : PK) -> Self :: Output { let primary_key = self . source . target () . primary_key () ; let predicate = self . fields (primary_key) . eq_all (id) ; QueryDsl :: filter (self , predicate) } }
};
}
