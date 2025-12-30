// Generated macro for impl_valid_grouping_for_tuple_of_columns (macro)
macro_rules! Depcrate_type_impls_tuplesimpl_valid_grouping_for_tuple_of_columns {
() => {
// Module: crate::type_impls::tuples
// Provides: {"impl_valid_grouping_for_tuple_of_columns"}
// Dependencies: {}
macro_rules ! impl_valid_grouping_for_tuple_of_columns { ($ T1 : ident , $ ($ T : ident ,) +) => { impl <$ T1 , $ ($ T ,) * __GroupByClause > ValidGrouping < __GroupByClause > for ($ T1 , $ ($ T ,) *) where $ T1 : ValidGrouping < __GroupByClause >, ($ ($ T ,) *) : ValidGrouping < __GroupByClause >, $ T1 :: IsAggregate : MixedAggregates << ($ ($ T ,) *) as ValidGrouping < __GroupByClause >>:: IsAggregate >, { type IsAggregate = <$ T1 :: IsAggregate as MixedAggregates << ($ ($ T ,) *) as ValidGrouping < __GroupByClause >>:: IsAggregate >>:: Output ; } impl <$ T1 , $ ($ T ,) * Col > IsContainedInGroupBy < Col > for ($ T1 , $ ($ T ,) *) where Col : Column , ($ ($ T ,) *) : IsContainedInGroupBy < Col >, $ T1 : IsContainedInGroupBy < Col >, $ T1 :: Output : is_contained_in_group_by :: IsAny << ($ ($ T ,) *) as IsContainedInGroupBy < Col >>:: Output > { type Output = <$ T1 :: Output as is_contained_in_group_by :: IsAny << ($ ($ T ,) *) as IsContainedInGroupBy < Col >>:: Output >>:: Output ; } } ; ($ T1 : ident ,) => { impl <$ T1 , Col > IsContainedInGroupBy < Col > for ($ T1 ,) where Col : Column , $ T1 : IsContainedInGroupBy < Col > { type Output = <$ T1 as IsContainedInGroupBy < Col >>:: Output ; } impl <$ T1 , __GroupByClause > ValidGrouping < __GroupByClause > for ($ T1 ,) where $ T1 : ValidGrouping < __GroupByClause > { type IsAggregate = $ T1 :: IsAggregate ; } } ; }
};
}
