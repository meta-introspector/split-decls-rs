// Generated macro for impl_2579 (impl)
macro_rules! Depcrate_pg_expression_arrayimpl_2579 {
() => {
// Module: crate::pg::expression::array
// Provides: {"impl_2579"}
// Dependencies: {}
impl < T , ST , GB > ValidGrouping < GB > for ArrayLiteral < T , ST > where T : ValidGrouping < GB > , { type IsAggregate = T :: IsAggregate ; }
};
}
