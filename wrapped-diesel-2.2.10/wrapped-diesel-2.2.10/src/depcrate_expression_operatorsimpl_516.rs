// Generated macro for impl_516 (impl)
macro_rules! Depcrate_expression_operatorsimpl_516 {
() => {
// Module: crate::expression::operators
// Provides: {"impl_516"}
// Dependencies: {}
impl < T , U > crate :: expression :: Expression for Like < T , U > where T : crate :: expression :: Expression , U : crate :: expression :: Expression , < T as crate :: expression :: Expression > :: SqlType : crate :: sql_types :: SqlType , < U as crate :: expression :: Expression > :: SqlType : crate :: sql_types :: SqlType , crate :: sql_types :: is_nullable :: IsSqlTypeNullable < < T as crate :: expression :: Expression > :: SqlType > : crate :: sql_types :: OneIsNullable < crate :: sql_types :: is_nullable :: IsSqlTypeNullable < < U as crate :: expression :: Expression > :: SqlType , > , > , crate :: sql_types :: is_nullable :: IsOneNullable < < T as crate :: expression :: Expression > :: SqlType , < U as crate :: expression :: Expression > :: SqlType , > : crate :: sql_types :: MaybeNullableType < crate :: sql_types :: Bool > , { type SqlType = crate :: sql_types :: is_nullable :: MaybeNullable < crate :: sql_types :: is_nullable :: IsOneNullable < < T as crate :: expression :: Expression > :: SqlType , < U as crate :: expression :: Expression > :: SqlType , > , crate :: sql_types :: Bool , > ; }
};
}
