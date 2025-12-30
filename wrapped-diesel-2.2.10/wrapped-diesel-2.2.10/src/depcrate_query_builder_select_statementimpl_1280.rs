// Generated macro for impl_1280 (impl)
macro_rules! Depcrate_query_builder_select_statementimpl_1280 {
() => {
// Module: crate::query_builder::select_statement
// Provides: {"impl_1280"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC > SelectStatement < F , S , D , W , O , LOf , G , H , LC > { # [allow (clippy :: too_many_arguments)] pub (crate) fn new (select : S , from : F , distinct : D , where_clause : W , order : O , limit_offset : LOf , group_by : G , having : H , locking : LC ,) -> Self { SelectStatement { select , from , distinct , where_clause , order , limit_offset , group_by , having , locking , } } }
};
}
