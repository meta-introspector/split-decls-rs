// Generated macro for impl_504 (impl)
macro_rules! Depcrate_expression_operatorsimpl_504 {
() => {
// Module: crate::expression::operators
// Provides: {"impl_504"}
// Dependencies: {}
impl < T , U > Insertable < T :: Table > for Eq < T , U > where T : Column , { type Values = ValuesClause < ColumnInsertValue < T , U > , T :: Table > ; fn values (self) -> Self :: Values { ValuesClause :: new (ColumnInsertValue :: new (self . right)) } }
};
}
