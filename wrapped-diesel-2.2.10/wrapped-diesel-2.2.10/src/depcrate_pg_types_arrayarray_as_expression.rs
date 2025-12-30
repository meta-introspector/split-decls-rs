// Generated macro for array_as_expression (macro)
macro_rules! Depcrate_pg_types_arrayarray_as_expression {
() => {
// Module: crate::pg::types::array
// Provides: {"array_as_expression"}
// Dependencies: {}
macro_rules ! array_as_expression { ($ ty : ty , $ sql_type : ty) => { # [cfg (feature = "postgres_backend")] # [allow (clippy :: extra_unused_lifetimes)] impl <'a , 'b , ST : 'static , T > AsExpression <$ sql_type > for $ ty { type Expression = Bound <$ sql_type , Self >; fn as_expression (self) -> Self :: Expression { Bound :: new (self) } } } ; }
};
}
